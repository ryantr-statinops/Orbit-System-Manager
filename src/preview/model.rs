#[derive(Clone, Debug, PartialEq)]
pub struct HistoryPoint {
    pub seconds_ago: u8,
    pub value: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessorPreview {
    pub id: usize,
    pub utilization: f32,
    pub history: Vec<HistoryPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CpuPreview {
    pub name: String,
    pub frequency_ghz: Option<f32>,
    pub physical_cores: Option<usize>,
    pub cores: Vec<ProcessorPreview>,
    pub average: Option<f32>,
    pub std_dev: Option<f32>,
    pub variance: Option<f32>,
    pub history: Vec<HistoryPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryPreview {
    pub used_gib: f32,
    pub total_gib: f32,
    pub history: Vec<HistoryPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GpuPreview {
    pub name: String,
    pub utilization: f32,
    pub vram_used_gib: f32,
    pub vram_total_gib: f32,
    pub temperature_c: f32,
    pub clock_mhz: f32,
    pub history: Vec<HistoryPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkPreview {
    pub received_kib_per_sec: f32,
    pub sent_kib_per_sec: f32,
    pub received_history: Vec<HistoryPoint>,
    pub sent_history: Vec<HistoryPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DashboardSnapshot {
    pub elapsed_seconds: u64,
    pub scenario: DemoScenario,
    pub cpu_name: String,
    pub cpu_frequency_ghz: Option<f32>,
    pub physical_cores: Option<usize>,
    pub cores: Vec<ProcessorPreview>,
    pub cpu_average: Option<f32>,
    pub cpu_std_dev: Option<f32>,
    pub cpu_variance: Option<f32>,
    pub cpu_history: Vec<HistoryPoint>,
    pub ram_used_gib: Option<f32>,
    pub ram_total_gib: Option<f32>,
    pub ram_history: Vec<HistoryPoint>,
    pub gpu: Option<GpuPreview>,
    pub network: Option<NetworkPreview>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DemoScenario {
    #[default]
    Full,
    Empty,
    Single,
    Unavailable,
}

impl DemoScenario {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "full" => Some(Self::Full),
            "empty" => Some(Self::Empty),
            "single" => Some(Self::Single),
            "unavailable" => Some(Self::Unavailable),
            _ => None,
        }
    }
}
