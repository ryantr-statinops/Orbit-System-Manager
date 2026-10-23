use sysinfo::{System, Networks};
use crate::telemetry::models::{
    CpuDetail, GpuMetric, MemoryDetail, NetworkSpeed, ProcessInfo, TelemetryPayload,
};

pub struct TelemetryCollector {
    sys: System,
    networks: Networks,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let networks = Networks::new_with_refreshed_list();
        Self { sys, networks }
    }

    pub fn collect(&mut self) -> TelemetryPayload {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.networks.refresh(true);

        // 1. CPU
        let cpus = self.sys.cpus();
        let mut cores: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();
        if cores.is_empty() {
            cores = vec![0.0; 16];
        }

        let avg: f32 = if !cores.is_empty() {
            cores.iter().sum::<f32>() / cores.len() as f32
        } else {
            0.0
        };

        let variance: f32 = if !cores.is_empty() {
            cores.iter().map(|c| (c - avg).powi(2)).sum::<f32>() / cores.len() as f32
        } else {
            0.0
        };
        let std_dev = variance.sqrt();

        // 2. Memory
        let total_bytes = self.sys.total_memory() as f64;
        let used_bytes = self.sys.used_memory() as f64;
        let total_gb = total_bytes / (1024.0 * 1024.0 * 1024.0);
        let used_gb = used_bytes / (1024.0 * 1024.0 * 1024.0);
        let percent = if total_gb > 0.0 {
            (used_gb / total_gb) * 100.0
        } else {
            0.0
        };

        // 3. Top processes by memory usage
        let mut processes: Vec<ProcessInfo> = self
            .sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                name: p.name().to_string_lossy().to_string(),
                memory_mb: (p.memory() as f64) / (1024.0 * 1024.0),
            })
            .collect();
        processes.sort_by(|a, b| {
            b.memory_mb
                .partial_cmp(&a.memory_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        processes.truncate(5);

        // 4. Network I/O
        let mut total_received_bytes = 0;
        let mut total_transmitted_bytes = 0;
        for (_interface_name, data) in &self.networks {
            total_received_bytes += data.received();
            total_transmitted_bytes += data.transmitted();
        }
        let received_kbps = (total_received_bytes as f64) / 1024.0;
        let sent_kbps = (total_transmitted_bytes as f64) / 1024.0;

        // 5. GPU placeholder (ready for OS-specific GPU provider trait)
        let gpu_detail = vec![
            GpuMetric {
                name: "Primary GPU".to_string(),
                load_percent: 0.0,
                temperature_c: 0.0,
            },
        ];

        TelemetryPayload {
            cpu_detail: CpuDetail {
                cores,
                avg_utilization: avg,
                std_dev,
                variance,
            },
            memory_detail: MemoryDetail {
                total_gb,
                used_gb,
                percent,
                processes,
            },
            gpu_detail,
            network_speed: NetworkSpeed {
                sent_kbps,
                received_kbps,
            },
        }
    }
}
