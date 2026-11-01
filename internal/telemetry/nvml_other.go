//go:build !windows

package telemetry

import "fmt"

func newNVIDIA() (GPUProvider, error) {
	return nil, fmt.Errorf("NVIDIA adapter not implemented on this OS in Stage 1A")
}
