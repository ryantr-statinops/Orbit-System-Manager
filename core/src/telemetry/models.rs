use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct CpuDetail {
    pub cores: Vec<f32>,
    pub avg_utilization: f32,
    pub std_dev: f32,
    pub variance: f32,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub memory_mb: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct MemoryDetail {
    pub total_gb: f64,
    pub used_gb: f64,
    pub percent: f64,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Serialize, Clone, Debug)]
pub struct GpuMetric {
    pub name: String,
    pub load_percent: f32,
    pub temperature_c: f32,
}

#[derive(Serialize, Clone, Debug)]
pub struct NetworkSpeed {
    pub sent_kbps: f64,
    pub received_kbps: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct TelemetryPayload {
    pub cpu_detail: CpuDetail,
    pub memory_detail: MemoryDetail,
    pub gpu_detail: Vec<GpuMetric>,
    pub network_speed: NetworkSpeed,
}
