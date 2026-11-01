package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"strings"
	"testing"
	"time"

	"github.com/ryantr-statinops/Orbit-System-Manager/internal/telemetry"
)

func TestInventoryJSONAndDiagnosticsAreSeparate(t *testing.T) {
	var out, diag bytes.Buffer
	err := inventoryCommand(context.Background(), []string{"inventory"}, &out, &diag, func(context.Context) telemetry.Inventory {
		return telemetry.Inventory{SchemaVersion: 1, Issues: []telemetry.Issue{{Source: "source", Status: telemetry.Error, Reason: "failed"}}}
	})
	if err != nil || !json.Valid(out.Bytes()) || !strings.Contains(diag.String(), "failed") {
		t.Fatalf("%v stdout=%s stderr=%s", err, &out, &diag)
	}
}

type fakeSampler struct {
	sequences []uint64
	closed    bool
}

func (f *fakeSampler) Collect(_ context.Context, seq uint64, at time.Time) telemetry.Sample {
	f.sequences = append(f.sequences, seq)
	return telemetry.Sample{SchemaVersion: 1, Sequence: seq, Timestamp: at.UTC(), Groups: map[string]telemetry.Group{
		"cpu": {Status: telemetry.WarmingUp, Metrics: map[string]telemetry.Metric{"utilization": telemetry.Missing(telemetry.WarmingUp, "percent", "test", "baseline", at)}},
		"gpu": {Status: telemetry.Unsupported, Reason: "adapter unavailable"},
	}}
}
func (f *fakeSampler) Close() error { f.closed = true; return nil }

func TestSampleCommandStreamsJSONLAndClosesCollector(t *testing.T) {
	fake := &fakeSampler{}
	var out, diag bytes.Buffer
	waits := 0
	wait := func(ctx context.Context, d time.Duration) (time.Time, error) { waits++; return time.Now(), nil }
	factory := func(time.Duration) sampleCollector { return fake }
	if err := sampleCommand(context.Background(), []string{"--interval", "2s", "--count", "2"}, &out, &diag, factory, wait); err != nil {
		t.Fatal(err)
	}
	lines := strings.Split(strings.TrimSpace(out.String()), "\n")
	if len(lines) != 2 || waits != 1 || !fake.closed || diag.Len() != 0 {
		t.Fatalf("lines=%d waits=%d close=%v stderr=%q", len(lines), waits, fake.closed, diag.String())
	}
	for i, line := range lines {
		var sample telemetry.Sample
		if err := json.Unmarshal([]byte(line), &sample); err != nil || sample.Sequence != uint64(i+1) {
			t.Fatalf("%s: %+v %v", line, sample, err)
		}
	}
	if !strings.Contains(lines[0], `"value":null`) || !strings.Contains(lines[0], `"status":"warming_up"`) {
		t.Fatal(lines[0])
	}
}

func TestSampleCommandRejectsOptionsBeforeCreatingCollector(t *testing.T) {
	for _, args := range [][]string{{"--interval", "3s"}, {"--count", "-1"}, {"--other"}, {"--count", "2", "extra"}} {
		created := false
		err := sampleCommand(context.Background(), args, &bytes.Buffer{}, &bytes.Buffer{}, func(time.Duration) sampleCollector { created = true; return &fakeSampler{} }, waitForSampleTick)
		if err == nil || created {
			t.Fatalf("args %v err=%v created=%v", args, err, created)
		}
	}
}

func TestSampleCommandStopsCleanlyOnCancellation(t *testing.T) {
	fake := &fakeSampler{}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	err := sampleCommand(ctx, []string{"--count", "1"}, &bytes.Buffer{}, &bytes.Buffer{}, func(time.Duration) sampleCollector { return fake }, func(context.Context, time.Duration) (time.Time, error) {
		return time.Time{}, errors.New("unexpected wait")
	})
	if err != nil || !fake.closed || len(fake.sequences) != 0 {
		t.Fatalf("err=%v fake=%+v", err, fake)
	}
}

func TestInventoryRejectsUnknownArguments(t *testing.T) {
	for _, args := range [][]string{nil, {"unknown"}, {"inventory", "extra"}} {
		if err := run(context.Background(), args, &bytes.Buffer{}, &bytes.Buffer{}); err == nil {
			t.Fatalf("accepted %v", args)
		}
	}
}
