package main

import (
	"context"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"os/signal"
	"time"

	"github.com/ryantr-statinops/Orbit-System-Manager/internal/uibench"
	"github.com/shirou/gopsutil/v4/cpu"
)

type sample struct {
	TimestampUTC        string   `json:"timestamp_utc"`
	ElapsedMilliseconds int64    `json:"elapsed_ms"`
	HostCPUPercent      *float64 `json:"host_cpu_percent"`
	HostCPUStatus       string   `json:"host_cpu_status"`
	ProcessTreeRSSBytes *uint64  `json:"process_tree_rss_bytes"`
	ProcessTreeCount    int      `json:"process_tree_process_count"`
	ProcessTreeStatus   string   `json:"process_tree_status"`
}

func main() {
	if err := run(os.Args[1:], os.Stdout, os.Stderr); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(2)
	}
}

func run(args []string, stdout, stderr io.Writer) error {
	flags := flag.NewFlagSet("ui-benchmark", flag.ContinueOnError)
	flags.SetOutput(stderr)
	pid := flags.Int("pid", 0, "PID of the Wails application to measure")
	interval := flags.Duration("interval", time.Second, "sample interval: 1s, 2s, or 5s")
	duration := flags.Duration("duration", 30*time.Minute, "measurement duration; 0 runs until Ctrl+C")
	if err := flags.Parse(args); err != nil {
		return err
	}
	if *pid <= 0 {
		return errors.New("--pid must be a positive process ID")
	}
	if *interval != time.Second && *interval != 2*time.Second && *interval != 5*time.Second {
		return errors.New("--interval must be 1s, 2s, or 5s")
	}
	if *duration < 0 {
		return errors.New("--duration cannot be negative")
	}

	root, err := uibench.NewProcessNode(int32(*pid))
	if err != nil {
		return fmt.Errorf("open target process: %w", err)
	}

	encoder := json.NewEncoder(stdout)
	started := time.Now()
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	for *duration == 0 || time.Since(started) < *duration {
		select {
		case <-ctx.Done():
			return nil
		default:
		}

		hostCPU, cpuErr := cpu.Percent((*interval).Seconds(), false)
		if cpuErr != nil {
			fmt.Fprintf(stderr, "read whole-machine CPU: %v\n", cpuErr)
		}

		tree := uibench.MeasureProcessTree(root)
		record := sample{
			TimestampUTC:        time.Now().UTC().Format(time.RFC3339Nano),
			ElapsedMilliseconds: time.Since(started).Milliseconds(),
			HostCPUStatus:       "error",
			ProcessTreeCount:    tree.ProcessCount,
			ProcessTreeStatus:   status(tree.Complete),
		}
		if cpuErr == nil && len(hostCPU) > 0 {
			record.HostCPUPercent = &hostCPU[0]
			record.HostCPUStatus = "available"
		}
		if tree.Complete {
			record.ProcessTreeRSSBytes = &tree.RSSBytes
		}
		if err := encoder.Encode(record); err != nil {
			return fmt.Errorf("write benchmark sample: %w", err)
		}
	}
	return nil
}

func status(complete bool) string {
	if complete {
		return "complete"
	}
	return "partial"
}
