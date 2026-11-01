package telemetry

import (
	"context"
	"errors"
	"github.com/shirou/gopsutil/v4/cpu"
	"testing"
	"time"
)

func TestCPUWarmupLoadIdleAndTopology(t *testing.T) {
	values := []cpu.TimesStat{{CPU: "0", User: 10, Idle: 10}}
	var readErr error
	c := CPUCollector{Interval: time.Second, Read: func(context.Context, bool) ([]cpu.TimesStat, error) { return values, readErr }}
	at := time.Now()
	if c.Collect(context.Background(), at).Status != WarmingUp {
		t.Fatal("first CPU sample")
	}
	values = []cpu.TimesStat{{CPU: "0", User: 11, Idle: 13}}
	g := c.Collect(context.Background(), at.Add(time.Second))
	if g.Status != Available || *g.Metrics["utilization"].Value != 25 {
		t.Fatalf("load: %+v", g)
	}
	values[0].Idle++
	g = c.Collect(context.Background(), at.Add(2*time.Second))
	if g.Status != Available || *g.Metrics["utilization"].Value != 0 {
		t.Fatal("idle")
	}
	values = append(values, cpu.TimesStat{CPU: "1", Idle: 1})
	if c.Collect(context.Background(), at.Add(3*time.Second)).Status != WarmingUp {
		t.Fatal("topology")
	}
	readErr = errors.New("read failed")
	if c.Collect(context.Background(), at.Add(4*time.Second)).Status != Error {
		t.Fatal("error")
	}
	readErr = nil
	if c.Collect(context.Background(), at.Add(5*time.Second)).Status != WarmingUp {
		t.Fatal("recovery")
	}
}
