package telemetry

import (
	"context"
	"github.com/shirou/gopsutil/v4/disk"
	"os"
	"testing"
	"time"
)

func TestDiskKeepsVolumesSeparateAndIOAlive(t *testing.T) {
	var read uint64 = 100
	c := DiskCollector{
		Partitions: func(context.Context, bool) ([]disk.PartitionStat, error) {
			return []disk.PartitionStat{{Mountpoint: "C:"}, {Mountpoint: "C:"}, {Mountpoint: "D:"}}, nil
		},
		Usage: func(_ context.Context, p string) (*disk.UsageStat, error) {
			if p == "D:" {
				return nil, os.ErrPermission
			}
			return &disk.UsageStat{Total: 1000, Used: 200, Free: 800, UsedPercent: 20}, nil
		},
		IO: func(context.Context, ...string) (map[string]disk.IOCountersStat, error) {
			return map[string]disk.IOCountersStat{"disk0": {ReadBytes: read, WriteBytes: 50}}, nil
		},
		Rates: RateTracker{Interval: time.Second},
	}
	at := time.Now()
	c.Collect(context.Background(), at)
	read += 200
	g := c.Collect(context.Background(), at.Add(time.Second))
	if len(g["disk_capacity"].Devices) != 2 || g["disk_capacity"].Devices[1].Metrics["total"].Status != PermissionDenied {
		t.Fatal(g)
	}
	if *g["disk_io"].Devices[0].Metrics["read_rate"].Value != 200 || len(g["disk_capacity"].Metrics) != 0 {
		t.Fatal(g)
	}
}
