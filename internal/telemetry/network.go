package telemetry

import (
	"context"
	gnet "github.com/shirou/gopsutil/v4/net"
	"sort"
	"time"
)

type NetworkCollector struct {
	Read  func(context.Context, bool) ([]gnet.IOCountersStat, error)
	Rates RateTracker
}

func (c *NetworkCollector) Collect(ctx context.Context, at time.Time) Group {
	read := c.Read
	if read == nil {
		read = gnet.IOCountersWithContext
	}
	stats, err := read(ctx, true)
	if err != nil {
		c.Rates.Reset()
		return FailedGroup(err)
	}
	sort.Slice(stats, func(i, j int) bool { return stats[i].Name < stats[j].Name })
	ids := make([]string, 0, len(stats))
	for _, s := range stats {
		ids = append(ids, s.Name)
	}
	c.Rates.Retain(ids)
	g := Group{Status: Available}
	for _, s := range stats {
		values, status, reason := c.Rates.Observe(s.Name, []uint64{s.BytesRecv, s.BytesSent}, at)
		m := map[string]Metric{}
		for i, key := range []string{"receive_rate", "send_rate"} {
			m[key] = Missing(status, "bytes/s", "gopsutil/net/io", reason, at)
			if status == Available {
				m[key] = Reading(values[i], "bytes/s", "gopsutil/net/io", at)
			}
		}
		g.Devices = append(g.Devices, DeviceMetrics{ID: s.Name, Metrics: m})
	}
	if len(stats) == 0 {
		g.Status, g.Reason = Unsupported, "no network counters"
	}
	return g
}
