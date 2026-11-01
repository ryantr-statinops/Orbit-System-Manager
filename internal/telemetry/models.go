// Package telemetry provides the collector PoC independently of its CLI.
package telemetry

import (
	"errors"
	"os"
	"strings"
	"time"
)

type Status string

const (
	Available        Status = "available"
	WarmingUp        Status = "warming_up"
	Unsupported      Status = "unsupported"
	PermissionDenied Status = "permission_denied"
	Error            Status = "error"
	Stale            Status = "stale"
)

// Metric uses null for missing data. Raw counters remain uint64 in collectors.
type Metric struct {
	Value     *float64  `json:"value"`
	Unit      string    `json:"unit"`
	Source    string    `json:"source"`
	Timestamp time.Time `json:"timestamp"`
	Status    Status    `json:"status"`
	Reason    string    `json:"reason,omitempty"`
}

func Reading(value float64, unit, source string, at time.Time) Metric {
	return Metric{Value: &value, Unit: unit, Source: source, Timestamp: at, Status: Available}
}

func Missing(status Status, unit, source, reason string, at time.Time) Metric {
	return Metric{Unit: unit, Source: source, Timestamp: at, Status: status, Reason: reason}
}

func ErrorStatus(err error) Status {
	var classified interface{ MetricStatus() Status }
	if errors.As(err, &classified) {
		return classified.MetricStatus()
	}
	var com interface{ Code() uintptr }
	if errors.As(err, &com) {
		switch uint32(com.Code()) {
		case 0x80070005, 0x80041003:
			return PermissionDenied
		case 0x8004100c, 0x80041010:
			return Unsupported
		}
	}
	// WMI's gopsutil wrapper may flatten COM access-denied HRESULTs to text.
	message := strings.ToLower(err.Error())
	if strings.Contains(message, "access denied") || strings.Contains(message, "access is denied") {
		return PermissionDenied
	}
	if errors.Is(err, os.ErrPermission) {
		return PermissionDenied
	}
	return Error
}

type DeviceMetrics struct {
	ID      string            `json:"id"`
	Metrics map[string]Metric `json:"metrics"`
}

type Group struct {
	Status  Status            `json:"status"`
	Reason  string            `json:"reason,omitempty"`
	Metrics map[string]Metric `json:"metrics,omitempty"`
	Devices []DeviceMetrics   `json:"devices,omitempty"`
}

func FailedGroup(err error) Group { return Group{Status: ErrorStatus(err), Reason: err.Error()} }

type Capability struct {
	Source         string `json:"source"`
	Implementation string `json:"implementation"`
	Validation     string `json:"validation"`
	Reason         string `json:"reason,omitempty"`
}

type Issue struct {
	Source string `json:"source"`
	Status Status `json:"status"`
	Reason string `json:"reason"`
}

type OSInfo struct {
	Name         string `json:"name"`
	Version      string `json:"version"`
	Build        string `json:"build"`
	Architecture string `json:"architecture"`
}

type CPUInfo struct {
	Name          string `json:"name"`
	LogicalCores  int    `json:"logical_cores"`
	PhysicalCores *int   `json:"physical_cores"`
}

type GPUDevice struct {
	ID     string `json:"id"`
	Name   string `json:"name"`
	Driver string `json:"driver"`
	Source string `json:"source"`
}

type Inventory struct {
	SchemaVersion int                   `json:"schema_version"`
	Timestamp     time.Time             `json:"timestamp"`
	OS            *OSInfo               `json:"os"`
	CPU           *CPUInfo              `json:"cpu"`
	RAMTotalBytes *uint64               `json:"ram_total_bytes"`
	GPUs          []GPUDevice           `json:"gpus"`
	Capabilities  map[string]Capability `json:"capabilities"`
	Issues        []Issue               `json:"issues"`
}

type Sample struct {
	SchemaVersion int              `json:"schema_version"`
	Sequence      uint64           `json:"sequence"`
	Timestamp     time.Time        `json:"timestamp"`
	Groups        map[string]Group `json:"groups"`
}
