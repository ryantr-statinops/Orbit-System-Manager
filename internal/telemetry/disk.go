package telemetry

import (
	"context"
	"github.com/shirou/gopsutil/v4/disk"
	"sort"
	"time"
)

type DiskCollector struct {
	Partitions func(context.Context, bool) ([]disk.PartitionStat, error)
	Usage      func(context.Context, string) (*disk.UsageStat, error)
	IO         func(context.Context, ...string) (map[string]disk.IOCountersStat, error)
	Rates      RateTracker
}

func (c *DiskCollector) Collect(ctx context.Context, at time.Time) map[string]Group {
	partitions, usage, readIO := c.Partitions, c.Usage, c.IO
	if partitions == nil {
		partitions = disk.PartitionsWithContext
	}
	if usage == nil {
		usage = disk.UsageWithContext
	}
	if readIO == nil {
		readIO = disk.IOCountersWithContext
	}
	groups := map[string]Group{}
	parts, err := partitions(ctx, false)
	if err != nil {
		groups["disk_capacity"] = FailedGroup(err)
	} else {
		g := Group{Status: Available}
		seen := map[string]bool{}
		sort.Slice(parts, func(i, j int) bool { return parts[i].Mountpoint < parts[j].Mountpoint })
		for _, p := range parts {
			if seen[p.Mountpoint] {
				continue
			}
			seen[p.Mountpoint] = true
			m := map[string]Metric{}
			u, e := usage(ctx, p.Mountpoint)
			if e != nil || u == nil {
				status, reason := Error, "no volume usage"
				if e != nil {
					status, reason = ErrorStatus(e), e.Error()
				}
				for _, key := range []string{"total", "used", "free", "utilization"} {
					unit := "bytes"
					if key == "utilization" {
						unit = "percent"
					}
					m[key] = Missing(status, unit, "gopsutil/disk/usage", reason, at)
				}
			} else {
				m["total"] = Reading(float64(u.Total), "bytes", "gopsutil/disk/usage", at)
				m["used"] = Reading(float64(u.Used), "bytes", "gopsutil/disk/usage", at)
				m["free"] = Reading(float64(u.Free), "bytes", "gopsutil/disk/usage", at)
				m["utilization"] = Reading(u.UsedPercent, "percent", "gopsutil/disk/usage", at)
			}
			g.Devices = append(g.Devices, DeviceMetrics{ID: p.Mountpoint, Metrics: m})
		}
		if len(g.Devices) == 0 {
			g.Status, g.Reason = Unsupported, "no mounted volumes"
		}
		groups["disk_capacity"] = g
	}
	stats, err := readIO(ctx)
	if err != nil {
		c.Rates.Reset()
		groups["disk_io"] = FailedGroup(err)
		return groups
	}
	ids := make([]string, 0, len(stats))
	for id := range stats {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	c.Rates.Retain(ids)
	g := Group{Status: Available}
	for _, id := range ids {
		s := stats[id]
		values, status, reason := c.Rates.Observe(id, []uint64{s.ReadBytes, s.WriteBytes}, at)
		m := map[string]Metric{}
		for i, key := range []string{"read_rate", "write_rate"} {
			m[key] = Missing(status, "bytes/s", "gopsutil/disk/io", reason, at)
			if status == Available {
				m[key] = Reading(values[i], "bytes/s", "gopsutil/disk/io", at)
			}
		}
		g.Devices = append(g.Devices, DeviceMetrics{ID: id, Metrics: m})
	}
	if len(ids) == 0 {
		g.Status, g.Reason = Unsupported, "no disk IO counters"
	}
	groups["disk_io"] = g
	return groups
}
