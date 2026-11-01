package telemetry

import (
	"fmt"
	"time"
)

type NVMLFailure struct {
	Operation string
	Code      uint32
}

func (e NVMLFailure) Error() string {
	return fmt.Sprintf("NVML %s returned code %d", e.Operation, e.Code)
}
func (e NVMLFailure) MetricStatus() Status {
	switch e.Code {
	case 3, 9, 12, 13:
		return Unsupported
	case 4:
		return PermissionDenied
	default:
		return Error
	}
}

func nvmlReading(value float64, unit string, err error, at time.Time) Metric {
	if err != nil {
		return Missing(ErrorStatus(err), unit, "NVML", err.Error(), at)
	}
	return Reading(value, unit, "NVML", at)
}

func mergeNVIDIAInventory(inv *Inventory, devices []GPUDevice) {
	// Replace the NVIDIA discovery view only after successful API enumeration.
	// Keep NVML IDs rather than guessing a WMI-to-NVML ordering relationship.
	kept := make([]GPUDevice, 0, len(inv.GPUs)+len(devices))
	for _, d := range inv.GPUs {
		if gpuVendor(d.Name) != "nvidia" {
			kept = append(kept, d)
		}
	}
	inv.GPUs = append(kept, devices...)
}
