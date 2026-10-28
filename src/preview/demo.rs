use super::model::DashboardSnapshot;
use super::{generate_cpu, generate_gpu, generate_memory, generate_network};

pub fn snapshot_at(elapsed_seconds: u64, core_count: usize) -> DashboardSnapshot {
    let cpu = generate_cpu(elapsed_seconds, core_count);
    let memory = generate_memory(elapsed_seconds);
    DashboardSnapshot {
        elapsed_seconds,
        cpu_name: cpu.name,
        cpu_frequency_ghz: cpu.frequency_ghz,
        physical_cores: cpu.physical_cores,
        cores: cpu.cores,
        cpu_average: cpu.average,
        cpu_std_dev: cpu.std_dev,
        cpu_variance: cpu.variance,
        cpu_history: cpu.history,
        ram_used_gib: Some(memory.used_gib),
        ram_total_gib: Some(memory.total_gib),
        ram_history: memory.history,
        gpu: Some(generate_gpu(elapsed_seconds)),
        network: Some(generate_network(elapsed_seconds)),
    }
}

pub struct DemoSource {
    elapsed_seconds: u64,
    core_count: usize,
}

impl DemoSource {
    pub fn new(core_count: usize) -> Self {
        Self {
            elapsed_seconds: 0,
            core_count,
        }
    }

    pub fn snapshot(&self) -> DashboardSnapshot {
        snapshot_at(self.elapsed_seconds, self.core_count)
    }

    pub fn tick(&mut self) -> DashboardSnapshot {
        self.elapsed_seconds = self.elapsed_seconds.saturating_add(1);
        self.snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::snapshot_at;

    #[test]
    fn full_snapshot_uses_one_elapsed_time_for_each_channel() {
        let snapshot = snapshot_at(18, 16);
        assert_eq!(snapshot.elapsed_seconds, 18);
        assert_eq!(snapshot.cores.len(), 16);
        assert_eq!(snapshot.cpu_history.len(), 61);
        assert_eq!(snapshot.ram_history.len(), 61);
        assert_eq!(snapshot.gpu.as_ref().unwrap().history.len(), 61);
        assert_eq!(
            snapshot.network.as_ref().unwrap().received_history.len(),
            61
        );
        assert_eq!(snapshot.cpu_history.last().unwrap().seconds_ago, 0);
        assert_eq!(snapshot.ram_history.last().unwrap().seconds_ago, 0);
        assert_eq!(
            snapshot.cpu_history.last().unwrap().value,
            snapshot.cpu_average
        );
        let gpu = snapshot.gpu.as_ref().unwrap();
        assert_eq!(gpu.history.last().unwrap().value, Some(gpu.utilization));
        let network = snapshot.network.as_ref().unwrap();
        assert_eq!(
            network.received_history.last().unwrap().value,
            Some(network.received_kib_per_sec)
        );
        assert_eq!(
            network.sent_history.last().unwrap().value,
            Some(network.sent_kib_per_sec)
        );
    }

    #[test]
    fn demo_source_advances_all_channels_on_one_second_ticks() {
        let mut source = super::DemoSource::new(16);
        assert_eq!(source.snapshot().elapsed_seconds, 0);

        let snapshot = source.tick();
        assert_eq!(snapshot.elapsed_seconds, 1);
        assert_eq!(
            snapshot.cpu_history.last().unwrap().value,
            snapshot.cpu_average
        );
        assert_eq!(
            snapshot.ram_history.last().unwrap().value,
            snapshot.ram_used_gib
        );
        assert_eq!(
            snapshot.gpu.as_ref().unwrap().history.last().unwrap().value,
            Some(snapshot.gpu.as_ref().unwrap().utilization)
        );
        assert_eq!(
            snapshot
                .network
                .as_ref()
                .unwrap()
                .received_history
                .last()
                .unwrap()
                .seconds_ago,
            0
        );
    }
}
