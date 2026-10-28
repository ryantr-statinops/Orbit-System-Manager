use super::model::{DashboardSnapshot, DemoScenario, HistoryPoint};
use super::{generate_cpu, generate_gpu, generate_memory, generate_network};

pub fn snapshot_at(elapsed_seconds: u64, core_count: usize) -> DashboardSnapshot {
    let cpu = generate_cpu(elapsed_seconds, core_count);
    let memory = generate_memory(elapsed_seconds);
    DashboardSnapshot {
        elapsed_seconds,
        scenario: DemoScenario::Full,
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

pub struct PreviewState {
    source: DemoSource,
    paused: bool,
    scenario: DemoScenario,
}

impl PreviewState {
    pub fn new(core_count: usize) -> Self {
        Self {
            source: DemoSource::new(core_count),
            paused: false,
            scenario: DemoScenario::Full,
        }
    }

    pub fn with_scenario(core_count: usize, scenario: DemoScenario) -> Self {
        Self {
            source: DemoSource::new(core_count),
            paused: false,
            scenario,
        }
    }

    pub fn snapshot(&self) -> DashboardSnapshot {
        apply_scenario(self.source.snapshot(), self.scenario)
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    pub fn tick(&mut self) -> Option<DashboardSnapshot> {
        (!self.paused && self.scenario == DemoScenario::Full).then(|| {
            self.source.tick();
            self.snapshot()
        })
    }
}

fn apply_scenario(mut snapshot: DashboardSnapshot, scenario: DemoScenario) -> DashboardSnapshot {
    snapshot.scenario = scenario;
    match scenario {
        DemoScenario::Full => {}
        DemoScenario::Single => {
            keep_latest(&mut snapshot.cpu_history);
            keep_latest(&mut snapshot.ram_history);
            for core in &mut snapshot.cores {
                keep_latest(&mut core.history);
            }
            if let Some(gpu) = &mut snapshot.gpu {
                keep_latest(&mut gpu.history);
            }
            if let Some(network) = &mut snapshot.network {
                keep_latest(&mut network.received_history);
                keep_latest(&mut network.sent_history);
            }
        }
        DemoScenario::Empty | DemoScenario::Unavailable => {
            snapshot.cpu_frequency_ghz = None;
            snapshot.physical_cores = None;
            snapshot.cores.clear();
            snapshot.cpu_average = None;
            snapshot.cpu_std_dev = None;
            snapshot.cpu_variance = None;
            snapshot.cpu_history.clear();
            snapshot.ram_used_gib = None;
            snapshot.ram_total_gib = None;
            snapshot.ram_history.clear();
            snapshot.gpu = None;
            snapshot.network = None;
        }
    }
    snapshot
}

fn keep_latest(points: &mut Vec<HistoryPoint>) {
    if let Some(latest) = points.pop() {
        points.clear();
        points.push(latest);
    }
}

#[cfg(test)]
mod tests {
    use super::{snapshot_at, DemoScenario};

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

    #[test]
    fn paused_preview_holds_the_clock_until_resumed() {
        let mut state = super::PreviewState::new(16);
        assert_eq!(state.tick().unwrap().elapsed_seconds, 1);

        assert!(state.toggle_pause());
        assert!(state.is_paused());
        assert!(state.tick().is_none());
        assert_eq!(state.snapshot().elapsed_seconds, 1);

        assert!(!state.toggle_pause());
        assert!(!state.is_paused());
        assert_eq!(state.tick().unwrap().elapsed_seconds, 2);
    }

    #[test]
    fn availability_scenarios_have_expected_history_shapes() {
        let full = super::PreviewState::new(4).snapshot();
        let single = super::PreviewState::with_scenario(4, DemoScenario::Single).snapshot();
        let empty = super::PreviewState::with_scenario(4, DemoScenario::Empty).snapshot();
        let unavailable =
            super::PreviewState::with_scenario(4, DemoScenario::Unavailable).snapshot();

        assert_eq!(full.cpu_history.len(), 61);
        assert_eq!(single.cpu_history.len(), 1);
        assert!(single.cores.iter().all(|core| core.history.len() == 1));
        assert_eq!(single.ram_history.len(), 1);
        assert_eq!(single.gpu.as_ref().unwrap().history.len(), 1);
        assert_eq!(single.network.as_ref().unwrap().received_history.len(), 1);
        assert!(empty.cpu_history.is_empty());
        assert!(empty.cpu_average.is_none());
        assert!(empty.gpu.is_none());
        assert_eq!(empty.scenario, DemoScenario::Empty);
        assert!(unavailable.cpu_history.is_empty());
        assert!(unavailable.gpu.is_none());
        assert_eq!(unavailable.scenario, DemoScenario::Unavailable);
    }
}
