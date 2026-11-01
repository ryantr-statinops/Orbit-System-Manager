package telemetry

import (
	"context"
	"fmt"
	"github.com/yusufpapurcu/wmi"
	"sort"
)

func discoverGPUs(ctx context.Context) ([]GPUDevice, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	var adapters []struct {
		Name          string
		DriverVersion string
	}
	if err := wmi.Query("SELECT Name, DriverVersion FROM Win32_VideoController", &adapters); err != nil {
		return nil, err
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	sort.Slice(adapters, func(i, j int) bool { return adapters[i].Name < adapters[j].Name })
	devices := make([]GPUDevice, 0, len(adapters))
	for i, a := range adapters {
		devices = append(devices, GPUDevice{ID: fmt.Sprintf("display-%d", i), Name: a.Name, Driver: a.DriverVersion, Source: "WMI/Win32_VideoController"})
	}
	return devices, nil
}
