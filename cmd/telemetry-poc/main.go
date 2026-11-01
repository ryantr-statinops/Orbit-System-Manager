package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/signal"
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
	return inventoryCommand(ctx, args, out, diagnostics, telemetry.CollectInventory)
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
