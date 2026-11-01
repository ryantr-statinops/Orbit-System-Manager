package telemetry

import (
	"context"
	"encoding/json"
	"errors"
	"strings"
	"testing"

	"github.com/shirou/gopsutil/v4/cpu"
	"github.com/shirou/gopsutil/v4/host"
	"github.com/shirou/gopsutil/v4/mem"
)

func TestInventoryOmitsMachineIdentifiersAndKeepsPartialErrors(t *testing.T) {
	src := InventorySources{
		Host: func(context.Context) (*host.InfoStat, error) {
			return &host.InfoStat{Hostname: "PRIVATE_HOST", HostID: "PRIVATE_ID", Platform: "test", PlatformVersion: "1", KernelVersion: "build"}, nil
		},
		CPU: func(context.Context) ([]cpu.InfoStat, error) { return []cpu.InfoStat{{ModelName: "CPU"}}, nil },
		Counts: func(_ context.Context, logical bool) (int, error) {
			if logical {
				return 4, nil
			}
			return 0, errors.New("physical unavailable")
		},
		Memory: func(context.Context) (*mem.VirtualMemoryStat, error) { return nil, errors.New("memory unavailable") },
	}
	inv := CollectInventoryFrom(context.Background(), src)
	if inv.CPU == nil || inv.CPU.LogicalCores != 4 || inv.CPU.PhysicalCores != nil || inv.RAMTotalBytes != nil || len(inv.Issues) != 2 {
		t.Fatalf("partial inventory: %+v", inv)
	}
	b, _ := json.Marshal(inv)
	if strings.Contains(string(b), "PRIVATE_") {
		t.Fatal("machine identifiers leaked")
	}
}
