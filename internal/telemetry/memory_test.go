package telemetry

import (
	"context"
	"github.com/shirou/gopsutil/v4/mem"
	"os"
	"testing"
	"time"
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
	}
	g := c.Collect(context.Background(), time.Now())
	if *g["memory"].Metrics["used"].Value != 600 || *g["memory"].Metrics["utilization"].Value != 60 || g["swap"].Status != PermissionDenied {
		t.Fatal(g)
	}
}
