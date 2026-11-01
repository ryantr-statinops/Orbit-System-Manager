package telemetry

import (
	"context"
	"fmt"
	"github.com/shirou/gopsutil/v4/sensors"
	"runtime"
	"time"
)

type SensorCollector struct {
	Read   func(context.Context) ([]sensors.TemperatureStat, error)
	last   time.Time
	cached Group
}

func (c *SensorCollector) Collect(ctx context.Context, at time.Time) Group {
	if !c.last.IsZero() && at.Sub(c.last) >= 0 && at.Sub(c.last) < 2*time.Second {
		return c.cached
	}
	read := c.Read
	if read == nil {
		if runtime.GOOS != "windows" {
			return Group{Status: Unsupported, Reason: "Linux sensor adapter not implemented in Stage 1A"}
		}
		read = sensors.TemperaturesWithContext
	}
	c.last = at
	values, err := read(ctx)
	if err != nil {
		c.cached = FailedGroup(err)
		return c.cached
	}
	g := Group{Status: Available}
	for i, v := range values {
		// Do not emit WMI instance identifiers or claim these are CPU readings.
		g.Devices = append(g.Devices, DeviceMetrics{ID: fmt.Sprintf("acpi-thermal-zone-%d", i), Metrics: map[string]Metric{
			"temperature": Reading(v.Temperature, "celsius", "gopsutil/ACPI thermal zone", at),
		}})
	}
	if len(values) == 0 {
		g.Status, g.Reason = Unsupported, "ACPI provider returned no thermal zones; CPU package sensor coverage is not established"
	}
	c.cached = g
	return g
}
