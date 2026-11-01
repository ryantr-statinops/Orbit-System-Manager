package telemetry

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/shirou/gopsutil/v4/mem"
)

type MemoryCollector struct {
	Virtual     func(context.Context) (*mem.VirtualMemoryStat, error)
	Swap        func(context.Context) (*mem.SwapMemoryStat, error)
	SwapDevices func(context.Context) ([]*mem.SwapDevice, error)
}

func (c MemoryCollector) Collect(ctx context.Context, at time.Time) map[string]Group {
	virtual, swap, swapDevices := c.Virtual, c.Swap, c.SwapDevices
	if virtual == nil {
		virtual = mem.VirtualMemoryWithContext
	}
	if swap == nil {
		swap = mem.SwapMemoryWithContext
	}
	if swapDevices == nil {
		swapDevices = mem.SwapDevicesWithContext
	}
	groups := map[string]Group{}
	if m, err := virtual(ctx); err != nil {
		groups["memory"] = FailedGroup(err)
	} else if m == nil || m.Total == 0 {
		groups["memory"] = FailedGroup(fmt.Errorf("no physical memory capacity"))
	} else {
		groups["memory"] = Group{Status: Available, Metrics: map[string]Metric{
			"total":       Reading(float64(m.Total), "bytes", "gopsutil/mem", at),
			"used":        Reading(float64(m.Used), "bytes", "gopsutil/mem", at),
			"available":   Reading(float64(m.Available), "bytes", "gopsutil/mem", at),
			"utilization": Reading(m.UsedPercent, "percent", "gopsutil/mem", at),
		}}
	}
	s, err := swap(ctx)
	swapSource := "gopsutil/mem/swap"
	if err != nil {
		devices, deviceErr := swapDevices(ctx)
		if deviceErr != nil {
			err = errors.Join(err, deviceErr)
		} else {
			s = sumSwapDevices(devices)
			err = nil
			swapSource = "gopsutil/mem/swap-devices"
		}
	}
	if err != nil {
		groups["swap"] = FailedGroup(err)
	} else if s == nil {
		groups["swap"] = FailedGroup(fmt.Errorf("no swap information"))
	} else {
		groups["swap"] = Group{Status: Available, Metrics: map[string]Metric{
			"total": Reading(float64(s.Total), "bytes", swapSource, at),
			"used":  Reading(float64(s.Used), "bytes", swapSource, at),
			"free":  Reading(float64(s.Free), "bytes", swapSource, at),
		}}
	}
	return groups
}

func sumSwapDevices(devices []*mem.SwapDevice) *mem.SwapMemoryStat {
	var used, free uint64
	for _, device := range devices {
		if device == nil {
			continue
		}
		used += device.UsedBytes
		free += device.FreeBytes
	}
	return &mem.SwapMemoryStat{Total: used + free, Used: used, Free: free}
}
