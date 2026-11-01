package telemetry

import (
	"context"
	"fmt"
	"runtime"
	"time"

	"github.com/shirou/gopsutil/v4/cpu"
	"github.com/shirou/gopsutil/v4/host"
	"github.com/shirou/gopsutil/v4/mem"
)

type InventorySources struct {
	Host   func(context.Context) (*host.InfoStat, error)
	CPU    func(context.Context) ([]cpu.InfoStat, error)
	Counts func(context.Context, bool) (int, error)
	Memory func(context.Context) (*mem.VirtualMemoryStat, error)
}

func CollectInventory(ctx context.Context) Inventory {
	inv := CollectInventoryFrom(ctx, InventorySources{host.InfoWithContext, cpu.InfoWithContext, cpu.CountsWithContext, mem.VirtualMemoryWithContext})
	addGPUInventory(ctx, &inv)
	return inv
}

func CollectInventoryFrom(ctx context.Context, src InventorySources) Inventory {
	inv := Inventory{SchemaVersion: 1, Timestamp: time.Now().UTC(), GPUs: []GPUDevice{}, Issues: []Issue{}, Capabilities: map[string]Capability{}}
	issue := func(source string, err error) {
		inv.Issues = append(inv.Issues, Issue{source, ErrorStatus(err), err.Error()})
	}
	if h, err := src.Host(ctx); err != nil {
		issue("gopsutil/host", err)
	} else if h != nil {
		inv.OS = &OSInfo{Name: h.Platform, Version: h.PlatformVersion, Build: h.KernelVersion, Architecture: runtime.GOARCH}
	} else {
		issue("gopsutil/host", fmt.Errorf("no host information"))
	}
	info, infoErr := src.CPU(ctx)
	logical, logicalErr := src.Counts(ctx, true)
	physical, physicalErr := src.Counts(ctx, false)
	if infoErr != nil {
		issue("gopsutil/cpu", infoErr)
	}
	if logicalErr != nil {
		issue("gopsutil/cpu/counts", logicalErr)
	}
	if physicalErr != nil {
		issue("gopsutil/cpu/physical", physicalErr)
	}
	if infoErr == nil && len(info) > 0 && logicalErr == nil && logical > 0 {
		inv.CPU = &CPUInfo{Name: info[0].ModelName, LogicalCores: logical}
		if physicalErr == nil && physical > 0 {
			inv.CPU.PhysicalCores = &physical
		}
	} else if infoErr == nil && logicalErr == nil {
		issue("gopsutil/cpu", fmt.Errorf("empty CPU inventory"))
	}
	if m, err := src.Memory(ctx); err != nil {
		issue("gopsutil/mem", err)
	} else if m != nil && m.Total > 0 {
		inv.RAMTotalBytes = &m.Total
	} else {
		issue("gopsutil/mem", fmt.Errorf("no memory capacity"))
	}
	for _, name := range []string{"cpu", "memory", "swap", "disk_capacity", "disk_io", "network"} {
		inv.Capabilities[name] = Capability{Source: "gopsutil", Implementation: "planned", Validation: "not_tested", Reason: "sampling collector not implemented yet"}
	}
	return inv
}
