mod chart;
mod cpu;
mod demo;
mod gpu;
mod memory;
mod model;
mod network;

pub use chart::{area_path_commands, line_path_commands, normalize_history, NormalizedPoint};
pub use cpu::generate_cpu;
pub use demo::{snapshot_at, DemoSource, PreviewState};
pub use gpu::generate_gpu;
pub use memory::generate_memory;
pub use model::{
    CpuPreview, DashboardSnapshot, DemoScenario, GpuPreview, HistoryPoint, MemoryPreview,
    NetworkPreview, ProcessorPreview,
};
pub use network::generate_network;
