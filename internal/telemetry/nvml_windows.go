package telemetry

import (
	"context"
	"fmt"
	"path/filepath"
	"runtime"
	"syscall"
	"time"
	"unsafe"

	"golang.org/x/sys/windows"
)

type nvmlMemory struct{ Total, Free, Used uint64 }
type nvmlUtilization struct{ GPU, Memory uint32 }
type nvidiaDevice struct {
	info   GPUDevice
	handle uintptr
}
type nvidiaSession struct {
	dll         windows.Handle
	initialized bool
	devices     []nvidiaDevice
}

func loadNVML() (windows.Handle, error) {
	dir, err := windows.GetSystemDirectory()
	if err != nil {
		return 0, err
	}
	paths := []string{filepath.Join(dir, "nvml.dll")}
	if programFiles, e := windows.KnownFolderPath(windows.FOLDERID_ProgramFiles, 0); e == nil {
		paths = append(paths, filepath.Join(programFiles, "NVIDIA Corporation", "NVSMI", "nvml.dll"))
	}
	for _, path := range paths {
		h, e := windows.LoadLibraryEx(path, 0, windows.LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR|windows.LOAD_LIBRARY_SEARCH_SYSTEM32)
		if e == nil {
			return h, nil
		}
		if e == windows.ERROR_ACCESS_DENIED {
			return 0, e
		}
	}
	return 0, NVMLFailure{"load from trusted system/driver directory", 12}
}

func (n *nvidiaSession) call(name string, args ...uintptr) error {
	address, err := windows.GetProcAddress(n.dll, name)
	if err != nil {
		return NVMLFailure{name, 13}
	}
	code, _, _ := syscall.SyscallN(address, args...)
	if code != 0 {
		return NVMLFailure{name, uint32(code)}
	}
	return nil
}

func newNVIDIA() (GPUProvider, error) {
	dll, err := loadNVML()
	if err != nil {
		return nil, err
	}
	n := &nvidiaSession{dll: dll}
	if err = n.call("nvmlInit_v2"); err != nil {
		n.Close()
		return nil, err
	}
	n.initialized = true
	var count uint32
	if err = n.call("nvmlDeviceGetCount_v2", uintptr(unsafe.Pointer(&count))); err != nil {
		n.Close()
		return nil, err
	}
	var driver [96]byte
	driverErr := n.call("nvmlSystemGetDriverVersion", uintptr(unsafe.Pointer(&driver[0])), uintptr(len(driver)))
	for i := uint32(0); i < count; i++ {
		var device uintptr
		if err = n.call("nvmlDeviceGetHandleByIndex_v2", uintptr(i), uintptr(unsafe.Pointer(&device))); err != nil {
			n.Close()
			return nil, err
		}
		var name [128]byte
		if err = n.call("nvmlDeviceGetName", device, uintptr(unsafe.Pointer(&name[0])), uintptr(len(name))); err != nil {
			n.Close()
			return nil, err
		}
		info := GPUDevice{ID: fmt.Sprintf("nvml-%d", i), Name: windows.ByteSliceToString(name[:]), Source: "NVML"}
		if driverErr == nil {
			info.Driver = windows.ByteSliceToString(driver[:])
		}
		n.devices = append(n.devices, nvidiaDevice{info, device})
	}
	return n, nil
}

func (n *nvidiaSession) Devices() []GPUDevice {
	devices := make([]GPUDevice, 0, len(n.devices))
	for _, d := range n.devices {
		devices = append(devices, d.info)
	}
	return devices
}

func (n *nvidiaSession) Collect(ctx context.Context, at time.Time) Group {
	if err := ctx.Err(); err != nil {
		return FailedGroup(err)
	}
	if n.dll == 0 {
		return Group{Status: Error, Reason: "NVML session closed"}
	}
	g := Group{Status: Available}
	for _, d := range n.devices {
		m := map[string]Metric{}
		var util nvmlUtilization
		err := n.call("nvmlDeviceGetUtilizationRates", d.handle, uintptr(unsafe.Pointer(&util)))
		m["utilization"] = nvmlReading(float64(util.GPU), "percent", err, at)
		var memory nvmlMemory
		err = n.call("nvmlDeviceGetMemoryInfo", d.handle, uintptr(unsafe.Pointer(&memory)))
		m["vram_used"] = nvmlReading(float64(memory.Used), "bytes", err, at)
		m["vram_total"] = nvmlReading(float64(memory.Total), "bytes", err, at)
		var temp, power uint32
		err = n.call("nvmlDeviceGetTemperature", d.handle, 0, uintptr(unsafe.Pointer(&temp)))
		m["temperature"] = nvmlReading(float64(temp), "celsius", err, at)
		err = n.call("nvmlDeviceGetPowerUsage", d.handle, uintptr(unsafe.Pointer(&power)))
		m["power"] = nvmlReading(float64(power)/1000, "watts", err, at)
		g.Devices = append(g.Devices, DeviceMetrics{ID: d.info.ID, Metrics: m})
	}
	if len(n.devices) == 0 {
		g.Status, g.Reason = Unsupported, "NVML enumerated no NVIDIA GPU"
	}
	runtime.KeepAlive(n)
	return g
}

func (n *nvidiaSession) Close() error {
	if n.dll == 0 {
		return nil
	}
	var shutdownErr error
	if n.initialized {
		shutdownErr = n.call("nvmlShutdown")
	}
	freeErr := windows.FreeLibrary(n.dll)
	n.dll = 0
	n.initialized = false
	if shutdownErr != nil {
		return shutdownErr
	}
	return freeErr
}
