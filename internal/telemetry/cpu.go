package telemetry

import (
	"context"
	"fmt"
	"runtime"
	"time"

	"github.com/shirou/gopsutil/v4/cpu"
)

type CPUCollector struct {
	Read     func(context.Context, bool) ([]cpu.TimesStat, error)
	Interval time.Duration
	previous map[string]cpu.TimesStat
	last     time.Time
}

func cpuTotals(c cpu.TimesStat) (float64, float64) {
	total := c.User + c.System + c.Idle + c.Nice + c.Iowait + c.Irq + c.Softirq + c.Steal
	// Guest counters are already included in user/nice on Linux.
	if runtime.GOOS != "linux" {
		total += c.Guest + c.GuestNice
	}
	return total, total - c.Idle - c.Iowait
}

func (c *CPUCollector) Collect(ctx context.Context, at time.Time) Group {
	read := c.Read
	if read == nil {
		read = cpu.TimesWithContext
	}
	current, err := read(ctx, true)
	if err != nil {
		c.previous = nil
		return FailedGroup(err)
	}
	if len(current) == 0 {
		c.previous = nil
		return Group{Status: Unsupported, Reason: "no logical CPU counters"}
	}
	prev := c.previous
	elapsed := at.Sub(c.last)
	c.previous = make(map[string]cpu.TimesStat, len(current))
	c.last = at
	valid := len(prev) == len(current) && elapsed > 0 && elapsed <= 3*c.Interval
	for _, v := range current {
		c.previous[v.CPU] = v
		if _, ok := prev[v.CPU]; !ok {
			valid = false
		}
	}
	g := Group{Status: Available, Metrics: map[string]Metric{}}
	var totalDelta, busyDelta float64
	for _, v := range current {
		m := Missing(WarmingUp, "percent", "gopsutil/cpu/times", "new baseline, topology change or sampling gap", at)
		if valid {
			total, busy := cpuTotals(v)
			oldTotal, oldBusy := cpuTotals(prev[v.CPU])
			dt, db := total-oldTotal, busy-oldBusy
			if dt <= 0 || db < 0 || db > dt+1e-6 {
				valid = false
			} else {
				totalDelta += dt
				busyDelta += db
				m = Reading(100*db/dt, "percent", "gopsutil/cpu/times", at)
			}
		}
		g.Devices = append(g.Devices, DeviceMetrics{ID: fmt.Sprintf("core-%s", v.CPU), Metrics: map[string]Metric{"utilization": m}})
	}
	if !valid || totalDelta <= 0 {
		g.Status = WarmingUp
		g.Metrics["utilization"] = Missing(WarmingUp, "percent", "gopsutil/cpu/times", "CPU baseline reset", at)
		for i := range g.Devices {
			g.Devices[i].Metrics["utilization"] = g.Metrics["utilization"]
		}
	} else {
		g.Metrics["utilization"] = Reading(100*busyDelta/totalDelta, "percent", "gopsutil/cpu/times", at)
	}
	return g
}
