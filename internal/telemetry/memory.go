package telemetry

import (
	"context"
	"fmt"
	"github.com/shirou/gopsutil/v4/mem"
	"time"
)

type MemoryCollector struct {
	Virtual func(context.Context) (*mem.VirtualMemoryStat, error)
	Swap    func(context.Context) (*mem.SwapMemoryStat, error)
}

func (c MemoryCollector) Collect(ctx context.Context, at time.Time) map[string]Group {
	virtual, swap := c.Virtual, c.Swap
	if virtual == nil {
		virtual = mem.VirtualMemoryWithContext
	}
	if swap == nil {
		swap = mem.SwapMemoryWithContext
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
	if s, err := swap(ctx); err != nil {
		groups["swap"] = FailedGroup(err)
	} else if s == nil {
		groups["swap"] = FailedGroup(fmt.Errorf("no swap information"))
	} else {
		groups["swap"] = Group{Status: Available, Metrics: map[string]Metric{
			"total": Reading(float64(s.Total), "bytes", "gopsutil/swap", at),
			"used":  Reading(float64(s.Used), "bytes", "gopsutil/swap", at),
			"free":  Reading(float64(s.Free), "bytes", "gopsutil/swap", at),
		}}
	}
	return groups
}
