package main

import (
	"bytes"
	"context"
	"encoding/json"
	"strings"
	"testing"

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

func TestInventoryRejectsUnknownArguments(t *testing.T) {
	for _, args := range [][]string{nil, {"unknown"}, {"inventory", "extra"}} {
		if err := run(context.Background(), args, &bytes.Buffer{}, &bytes.Buffer{}); err == nil {
			t.Fatalf("accepted %v", args)
		}
	}
}
