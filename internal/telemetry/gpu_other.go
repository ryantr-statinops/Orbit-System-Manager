//go:build !windows

package telemetry

import (
	"context"
	"fmt"
)

func discoverGPUs(context.Context) ([]GPUDevice, error) {
	return nil, fmt.Errorf("GPU discovery not implemented on this OS in Stage 1A")
}
