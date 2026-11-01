package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"os"
	"os/signal"
	"strings"
	"time"

	"github.com/ryantr-statinops/Orbit-System-Manager/internal/telemetry"
)

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()
	if err := run(ctx, os.Args[1:], os.Stdout, os.Stderr); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func run(ctx context.Context, args []string, out, diagnostics io.Writer) error {
	if len(args) == 0 {
		return fmt.Errorf("usage: telemetry-poc inventory | sample [--interval 1s|2s|5s] [--count N]")
	}
	switch args[0] {
	case "inventory":
		return inventoryCommand(ctx, args, out, diagnostics, telemetry.CollectInventory)
	case "sample":
		factory := func(interval time.Duration) sampleCollector { return telemetry.NewCollector(interval) }
		return sampleCommand(ctx, args[1:], out, diagnostics, factory, waitForSampleTick)
	default:
		return fmt.Errorf("unknown command %q; use inventory or sample", args[0])
	}
}

type sampleCollector interface {
	Collect(context.Context, uint64, time.Time) telemetry.Sample
	Close() error
}
type collectorFactory func(time.Duration) sampleCollector
type sampleWaiter func(context.Context, time.Duration) (time.Time, error)

type sampleOptions struct {
	interval time.Duration
	count    uint64
}

func parseSampleOptions(args []string) (sampleOptions, error) {
	opts := sampleOptions{interval: time.Second, count: 60}
	fs := flag.NewFlagSet("sample", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	fs.DurationVar(&opts.interval, "interval", time.Second, "sample period: 1s, 2s, or 5s")
	count := fs.Uint64("count", 60, "number of samples; 0 runs until interrupted")
	if err := fs.Parse(args); err != nil {
		return opts, fmt.Errorf("invalid sample options: %w", err)
	}
	if fs.NArg() != 0 {
		return opts, fmt.Errorf("unexpected sample arguments: %s", strings.Join(fs.Args(), " "))
	}
	if opts.interval != time.Second && opts.interval != 2*time.Second && opts.interval != 5*time.Second {
		return opts, fmt.Errorf("interval must be 1s, 2s, or 5s")
	}
	opts.count = *count
	return opts, nil
}

func sampleCommand(ctx context.Context, args []string, out, diagnostics io.Writer, factory collectorFactory, wait sampleWaiter) error {
	opts, err := parseSampleOptions(args)
	if err != nil {
		return err
	}
	c := factory(opts.interval)
	enc := json.NewEncoder(out)
	err = nil
	for seq := uint64(1); opts.count == 0 || seq <= opts.count; seq++ {
		if err := ctx.Err(); err != nil {
			break
		}
		if err := enc.Encode(c.Collect(ctx, seq, time.Now())); err != nil {
			err = fmt.Errorf("write sample: %w", err)
			break
		}
		if opts.count > 0 && seq == opts.count {
			break
		}
		if _, err := wait(ctx, opts.interval); err != nil {
			if ctx.Err() != nil {
				break
			}
			err = fmt.Errorf("wait for next sample: %w", err)
			break
		}
	}
	if closeErr := c.Close(); closeErr != nil {
		fmt.Fprintf(diagnostics, "collector shutdown: %s\n", closeErr)
		if err == nil {
			err = fmt.Errorf("collector shutdown: %w", closeErr)
		}
	}
	return err
}

func waitForSampleTick(ctx context.Context, interval time.Duration) (time.Time, error) {
	timer := time.NewTimer(interval)
	defer timer.Stop()
	select {
	case at := <-timer.C:
		return at, nil
	case <-ctx.Done():
		return time.Time{}, ctx.Err()
	}
}

func inventoryCommand(ctx context.Context, args []string, out, diagnostics io.Writer, collect func(context.Context) telemetry.Inventory) error {
	if len(args) != 1 || args[0] != "inventory" {
		return fmt.Errorf("usage: telemetry-poc inventory")
	}
	ctx, cancel := context.WithTimeout(ctx, 10*time.Second)
	defer cancel()
	inv := collect(ctx)
	for _, issue := range inv.Issues {
		fmt.Fprintf(diagnostics, "%s: %s (%s)\n", issue.Source, issue.Reason, issue.Status)
	}
	enc := json.NewEncoder(out)
	enc.SetIndent("", "  ")
	return enc.Encode(inv)
}
