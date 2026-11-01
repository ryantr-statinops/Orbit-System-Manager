package telemetry

import (
	"context"
	"github.com/shirou/gopsutil/v4/sensors"
	"testing"
	"time"
)

type hresultError uintptr

func (h hresultError) Error() string { return "WMI access failure" }
func (h hresultError) Code() uintptr { return uintptr(h) }

func TestSensorsThrottlePreserveTimestampAndLabel(t *testing.T) {
	calls := 0
	c := SensorCollector{Read: func(context.Context) ([]sensors.TemperatureStat, error) {
		calls++
		return []sensors.TemperatureStat{{SensorKey: "PRIVATE_INSTANCE", Temperature: 45}}, nil
	}}
	at := time.Now()
	c.Collect(context.Background(), at)
	g := c.Collect(context.Background(), at.Add(time.Second))
	m := g.Devices[0].Metrics["temperature"]
	if calls != 1 || !m.Timestamp.Equal(at) || m.Source != "gopsutil/ACPI thermal zone" || g.Devices[0].ID != "acpi-thermal-zone-0" {
		t.Fatal(g, calls)
	}
	c.Collect(context.Background(), at.Add(2*time.Second))
	if calls != 2 {
		t.Fatal(calls)
	}
}

func TestSensorsPermissionVersusNoReadings(t *testing.T) {
	c := SensorCollector{Read: func(context.Context) ([]sensors.TemperatureStat, error) { return nil, hresultError(0x80041003) }}
	at := time.Now()
	if c.Collect(context.Background(), at).Status != PermissionDenied {
		t.Fatal("WMI permission")
	}
	c.Read = func(context.Context) ([]sensors.TemperatureStat, error) { return nil, nil }
	if c.Collect(context.Background(), at.Add(2*time.Second)).Status != Unsupported {
		t.Fatal("empty source")
	}
}
