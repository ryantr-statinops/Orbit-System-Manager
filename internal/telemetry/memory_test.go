package telemetry

import (
	"context"
	"errors"
	"os"
	"testing"
	"time"

	"github.com/shirou/gopsutil/v4/mem"
)

func TestMemoryFailureDoesNotHideSwap(t *testing.T) {
	c := MemoryCollector{
		Virtual: func(context.Context) (*mem.VirtualMemoryStat, error) { return nil, os.ErrPermission },
		Swap:    func(context.Context) (*mem.SwapMemoryStat, error) { return &mem.SwapMemoryStat{}, nil },
	}
	g := c.Collect(context.Background(), time.Now())
	if g["memory"].Status != PermissionDenied || g["swap"].Status != Available || *g["swap"].Metrics["total"].Value != 0 {
		t.Fatalf("%+v", g)
	}
}

func TestMemoryKeepsSourceDefinition(t *testing.T) {
	c := MemoryCollector{
		Virtual: func(context.Context) (*mem.VirtualMemoryStat, error) {
			return &mem.VirtualMemoryStat{Total: 1000, Used: 600, Available: 300, UsedPercent: 60}, nil
		},
		Swap: func(context.Context) (*mem.SwapMemoryStat, error) { return nil, os.ErrPermission },
		SwapDevices: func(context.Context) ([]*mem.SwapDevice, error) {
			return nil, os.ErrPermission
		},
	}
	g := c.Collect(context.Background(), time.Now())
	if *g["memory"].Metrics["used"].Value != 600 || *g["memory"].Metrics["utilization"].Value != 60 || g["swap"].Status != PermissionDenied {
		t.Fatal(g)
	}
}

func TestSwapFallsBackToPageFileDevices(t *testing.T) {
	c := MemoryCollector{
		Virtual: func(context.Context) (*mem.VirtualMemoryStat, error) {
			return &mem.VirtualMemoryStat{Total: 1000}, nil
		},
		Swap: func(context.Context) (*mem.SwapMemoryStat, error) {
			return nil, errors.New("counter unavailable")
		},
		SwapDevices: func(context.Context) ([]*mem.SwapDevice, error) {
			return []*mem.SwapDevice{
				{UsedBytes: 40, FreeBytes: 60},
				{UsedBytes: 10, FreeBytes: 90},
			}, nil
		},
	}
	g := c.Collect(context.Background(), time.Now())
	if g["swap"].Status != Available {
		t.Fatalf("swap status = %s: %s", g["swap"].Status, g["swap"].Reason)
	}
	metrics := g["swap"].Metrics
	if *metrics["total"].Value != 200 || *metrics["used"].Value != 50 || *metrics["free"].Value != 150 {
		t.Fatalf("unexpected swap sums: %+v", metrics)
	}
	if metrics["total"].Source != "gopsutil/mem/swap-devices" {
		t.Fatalf("unexpected source: %s", metrics["total"].Source)
	}
}
