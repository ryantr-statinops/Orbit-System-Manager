mod model;
mod cpu;
mod memory;
mod gpu;

pub use cpu::generate_cpu;
pub use memory::generate_memory;
pub use gpu::generate_gpu;
pub use model::{CpuPreview, DashboardSnapshot, GpuPreview, HistoryPoint, MemoryPreview, NetworkPreview, ProcessorPreview};
