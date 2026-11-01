package telemetry

import (
	"testing"
	"time"
)

func TestNVMLFailuresStayPerMetric(t *testing.T) {
	for code, want := range map[uint32]Status{3: Unsupported, 4: PermissionDenied, 9: Unsupported, 12: Unsupported, 13: Unsupported, 15: Error} {
		m := nvmlReading(0, "watts", NVMLFailure{"test", code}, time.Now())
		if m.Value != nil || m.Status != want {
			t.Fatalf("code %d: %+v", code, m)
		}
	}
	if nvmlReading(0, "percent", nil, time.Now()).Value == nil {
		t.Fatal("zero is available")
	}
}

func TestNVMLInventoryDoesNotGuessWMIMapping(t *testing.T) {
	inv := Inventory{GPUs: []GPUDevice{{ID: "display-0", Name: "NVIDIA board"}, {ID: "display-1", Name: "Intel board"}}}
	mergeNVIDIAInventory(&inv, []GPUDevice{{ID: "nvml-0", Name: "board", Source: "NVML"}})
	if len(inv.GPUs) != 2 || inv.GPUs[0].ID != "display-1" || inv.GPUs[1].ID != "nvml-0" {
		t.Fatal(inv.GPUs)
	}
}
