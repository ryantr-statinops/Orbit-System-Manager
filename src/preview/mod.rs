mod model;
mod cpu;

pub use cpu::generate_cpu;
pub use model::{CpuPreview, DashboardSnapshot, GpuPreview, HistoryPoint, NetworkPreview, ProcessorPreview};
