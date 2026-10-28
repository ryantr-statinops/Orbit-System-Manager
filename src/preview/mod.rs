mod cpu;
mod demo;
mod gpu;
mod memory;
mod model;
mod network;

pub use cpu::generate_cpu;
pub use demo::{snapshot_at, DemoSource};
pub use gpu::generate_gpu;
pub use memory::generate_memory;
pub use model::{
    CpuPreview, DashboardSnapshot, GpuPreview, HistoryPoint, MemoryPreview, NetworkPreview,
    ProcessorPreview,
};
pub use network::generate_network;
