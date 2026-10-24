use serde::Serialize;

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetricStatus {
    WarmingUp,
    Available,
    Unavailable,
}

#[derive(Serialize, Clone, Debug)]
pub struct CpuDetail {
    pub cores: Vec<f32>,
    pub avg_utilization: Option<f32>,
    pub std_dev: Option<f32>,
    pub variance: Option<f32>,
    pub name: Option<String>,
    pub frequency_ghz: Option<f32>,
    pub physical_core_count: Option<usize>,
    pub status: MetricStatus,
}

#[derive(Serialize, Clone, Debug)]
pub struct MemoryDetail {
    pub total_gib: f64,
    pub used_gib: f64,
    pub percent: Option<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct GpuMetric {
    pub name: String,
    pub load_percent: Option<f32>,
    pub temperature_c: Option<f32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct NetworkSpeed {
    pub sent_kib_per_sec: Option<f64>,
    pub received_kib_per_sec: Option<f64>,
    pub status: MetricStatus,
}

#[derive(Serialize, Clone, Debug)]
pub struct TelemetryPayload {
    pub cpu_detail: CpuDetail,
    pub memory_detail: MemoryDetail,
    pub gpu_detail: Option<GpuMetric>,
    pub network_speed: NetworkSpeed,
}
