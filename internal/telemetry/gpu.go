package telemetry

import (
	"context"
	"strings"
	"time"
)

// GPUProvider owns its API session. IDs are local source IDs, not GPU UUIDs.
type GPUProvider interface {
	Devices() []GPUDevice
	Collect(context.Context, time.Time) Group
	Close() error
}

func gpuVendor(name string) string {
	n := strings.ToLower(name)
	for _, vendor := range []string{"nvidia", "intel", "amd"} {
		if strings.Contains(n, vendor) {
			return vendor
		}
	}
	if strings.Contains(n, "radeon") {
		return "amd"
	}
	return "unknown"
}

func addGPUInventory(ctx context.Context, inv *Inventory) {
	devices, err := discoverGPUs(ctx)
	inv.Capabilities["gpu"] = Capability{Source: "OS display inventory", Implementation: "discovery_only", Validation: "not_tested", Reason: "device discovery does not establish metric support"}
	if err != nil {
		inv.Issues = append(inv.Issues, Issue{"gpu/discovery", ErrorStatus(err), err.Error()})
	} else {
		inv.GPUs = devices
	}
	provider, nvmlErr := newNVIDIA()
	inv.Capabilities["nvidia"] = Capability{Source: "NVML", Implementation: "implemented_windows", Validation: "not_tested"}
	if nvmlErr != nil {
		inv.Capabilities["nvidia"] = Capability{Source: "NVML", Implementation: "implemented_windows", Validation: "not_tested", Reason: nvmlErr.Error()}
		inv.Issues = append(inv.Issues, Issue{"NVML", ErrorStatus(nvmlErr), nvmlErr.Error()})
		return
	}
	defer provider.Close()
	if len(provider.Devices()) > 0 {
		mergeNVIDIAInventory(inv, provider.Devices())
	}
}
