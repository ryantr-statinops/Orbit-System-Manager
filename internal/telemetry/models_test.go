package telemetry

import (
	"encoding/json"
	"errors"
	"os"
	"strings"
	"testing"
	"time"
)

func TestMissingIsNullAndZeroIsAReading(t *testing.T) {
	for _, status := range []Status{WarmingUp, Unsupported, PermissionDenied, Error, Stale} {
		b, err := json.Marshal(Missing(status, "percent", "test", "absent", time.Unix(0, 0)))
		if err != nil || !strings.Contains(string(b), `"value":null`) {
			t.Fatalf("%s: %s %v", status, b, err)
		}
	}
	m := Reading(0, "percent", "test", time.Now())
	if m.Value == nil || *m.Value != 0 || m.Status != Available {
		t.Fatal("zero must remain available")
	}
}

func TestWrappedPermissionError(t *testing.T) {
	if ErrorStatus(&os.PathError{Op: "read", Path: "sensor", Err: os.ErrPermission}) != PermissionDenied {
		t.Fatal("permission classification")
	}
	if ErrorStatus(errors.New("failed")) != Error {
		t.Fatal("generic classification")
	}
	if ErrorStatus(errors.New("Exception occurred. (Access denied )")) != PermissionDenied {
		t.Fatal("flattened WMI access denial")
	}
}
