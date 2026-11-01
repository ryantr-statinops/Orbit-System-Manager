package telemetry

import (
	"context"
	"time"
)

type Collector struct {
	interval time.Duration
	cpu      CPUCollector
	memory   MemoryCollector
	disk     DiskCollector
	network  NetworkCollector
	sensors  SensorCollector
	gpu      GPUProvider
	gpuErr   error
}

func NewCollector(interval time.Duration) *Collector {
	gpu, err := newNVIDIA()
	return &Collector{
		interval: interval,
		cpu:      CPUCollector{Interval: interval},
		disk:     DiskCollector{Rates: RateTracker{Interval: interval}},
		network:  NetworkCollector{Rates: RateTracker{Interval: interval}},
		gpu:      gpu, gpuErr: err,
	}
}

func (c *Collector) Collect(ctx context.Context, sequence uint64, at time.Time) Sample {
	groups := map[string]Group{
		"cpu":           c.cpu.Collect(ctx, at),
		"disk_capacity": {},
		"memory":        {}, "swap": {},
		"disk_io": {}, "network": c.network.Collect(ctx, at),
		"sensors": c.sensors.Collect(ctx, at),
	}
	for name, group := range c.memory.Collect(ctx, at) {
		groups[name] = group
	}
	for name, group := range c.disk.Collect(ctx, at) {
		groups[name] = group
	}
	if c.gpuErr != nil {
		groups["gpu"] = FailedGroup(c.gpuErr)
	} else if c.gpu != nil {
		groups["gpu"] = c.gpu.Collect(ctx, at)
	} else {
		groups["gpu"] = Group{Status: Unsupported, Reason: "NVIDIA provider unavailable"}
	}
	return Sample{SchemaVersion: 1, Sequence: sequence, Timestamp: at.UTC(), Groups: groups}
}

func (c *Collector) Close() error {
	if c.gpu == nil {
		return nil
	}
	return c.gpu.Close()
}
