package telemetry

import "testing"

func TestGPUVendorDoesNotImplyMetricSupport(t *testing.T) {
	for name, want := range map[string]string{"NVIDIA board": "nvidia", "Intel graphics": "intel", "Radeon board": "amd", "Virtual adapter": "unknown"} {
		if gpuVendor(name) != want {
			t.Fatalf("%s: %s", name, gpuVendor(name))
		}
	}
}
