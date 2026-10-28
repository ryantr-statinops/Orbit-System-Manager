mod model;
mod cpu;
mod memory;

pub use cpu::generate_cpu;
pub use memory::generate_memory;
pub use model::{CpuPreview, DashboardSnapshot, GpuPreview, HistoryPoint, MemoryPreview, NetworkPreview, ProcessorPreview};
