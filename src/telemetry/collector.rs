use crate::telemetry::models::{
    CpuDetail, MemoryDetail, MetricStatus, NetworkSpeed, TelemetryPayload,
};
use std::collections::HashSet;
use std::time::{Duration, Instant};
use sysinfo::{Networks, System};

pub struct TelemetryCollector {
    sys: System,
    networks: Networks,
    network_rates: NetworkRateTracker,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        sys.refresh_cpu_frequency();
        sys.refresh_memory();
        let networks = Networks::new_with_refreshed_list();
        Self {
            sys,
            networks,
            network_rates: NetworkRateTracker::default(),
        }
    }

    pub fn collect(&mut self) -> TelemetryPayload {
        self.sys.refresh_cpu_all();
        self.sys.refresh_cpu_frequency();
        self.sys.refresh_memory();
        self.networks.refresh(true);

        // 1. CPU
        let cpus = self.sys.cpus();
        let cores: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();

        let avg: Option<f32> = if !cores.is_empty() {
            Some(cores.iter().sum::<f32>() / cores.len() as f32)
        } else {
            None
        };

        let variance = avg
            .map(|avg| cores.iter().map(|c| (c - avg).powi(2)).sum::<f32>() / cores.len() as f32);
        let std_dev = variance.map(f32::sqrt);

        let frequencies: Vec<u64> = cpus
            .iter()
            .map(|cpu| cpu.frequency())
            .filter(|&f| f > 0)
            .collect();
        let frequency_ghz = (!frequencies.is_empty())
            .then(|| frequencies.iter().sum::<u64>() as f32 / frequencies.len() as f32 / 1000.0);

        // 2. Memory
        let total_bytes = self.sys.total_memory() as f64;
        let used_bytes = self.sys.used_memory() as f64;
        let total_gib = total_bytes / (1024.0 * 1024.0 * 1024.0);
        let used_gib = used_bytes / (1024.0 * 1024.0 * 1024.0);
        let percent = if total_gib > 0.0 {
            Some((used_gib / total_gib) * 100.0)
        } else {
            None
        };

        let now = Instant::now();
        let interface_names = self.networks.keys().cloned().collect::<HashSet<_>>();
        let (received_bytes, sent_bytes) =
            self.networks.values().fold((0u64, 0u64), |sum, data| {
                (
                    sum.0.saturating_add(data.received()),
                    sum.1.saturating_add(data.transmitted()),
                )
            });
        let (rates, network_status) =
            self.network_rates
                .update(now, interface_names, received_bytes, sent_bytes);
        let cpu_status = if cores.is_empty() {
            MetricStatus::Unavailable
        } else {
            MetricStatus::Available
        };

        TelemetryPayload {
            cpu_detail: CpuDetail {
                cores,
                avg_utilization: avg,
                std_dev,
                variance,
                name: cpus
                    .first()
                    .map(|cpu| cpu.brand().trim().to_owned())
                    .filter(|name| !name.is_empty()),
                frequency_ghz,
                physical_core_count: self.sys.physical_core_count(),
                status: cpu_status,
            },
            memory_detail: MemoryDetail {
                total_gib,
                used_gib,
                percent,
            },
            gpu_detail: None,
            network_speed: NetworkSpeed {
                sent_kib_per_sec: rates.map(|r| r.0),
                received_kib_per_sec: rates.map(|r| r.1),
                status: network_status,
            },
        }
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
struct NetworkRateTracker {
    last_sample: Option<(Instant, HashSet<String>)>,
}

impl NetworkRateTracker {
    fn update(
        &mut self,
        now: Instant,
        interfaces: HashSet<String>,
        received_bytes: u64,
        sent_bytes: u64,
    ) -> (Option<(f64, f64)>, MetricStatus) {
        let previous = self.last_sample.replace((now, interfaces.clone()));
        if interfaces.is_empty() {
            return (None, MetricStatus::Unavailable);
        }
        let Some((previous_at, previous_interfaces)) = previous else {
            return (None, MetricStatus::WarmingUp);
        };
        let elapsed = now.saturating_duration_since(previous_at);
        if elapsed.is_zero()
            || elapsed > Duration::from_secs(5)
            || interfaces != previous_interfaces
        {
            return (None, MetricStatus::WarmingUp);
        }
        let seconds = elapsed.as_secs_f64();
        (
            Some((
                sent_bytes as f64 / seconds / 1024.0,
                received_bytes as f64 / seconds / 1024.0,
            )),
            MetricStatus::Available,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interfaces() -> HashSet<String> {
        ["eth0".to_owned()].into_iter().collect()
    }

    #[test]
    fn network_rate_respects_elapsed_time() {
        for (milliseconds, expected) in [(500, 2.0), (1000, 1.0), (2000, 0.5)] {
            let start = Instant::now();
            let mut tracker = NetworkRateTracker::default();
            assert_eq!(tracker.update(start, interfaces(), 0, 0).0, None);
            let (rate, status) = tracker.update(
                start + Duration::from_millis(milliseconds),
                interfaces(),
                1024,
                2048,
            );
            assert_eq!(status, MetricStatus::Available);
            let (sent, received) = rate.unwrap();
            assert!((sent - expected * 2.0).abs() < 0.001);
            assert!((received - expected).abs() < 0.001);
        }
        let start = Instant::now();
        let mut tracker = NetworkRateTracker::default();
        tracker.update(start, interfaces(), 0, 0);
        assert_eq!(tracker.update(start, interfaces(), 1024, 1024).0, None);
    }

    #[test]
    fn network_rate_warms_up_after_first_sample_gaps_and_interface_changes() {
        let start = Instant::now();
        let mut tracker = NetworkRateTracker::default();
        assert_eq!(
            tracker.update(start, interfaces(), 10, 10).1,
            MetricStatus::WarmingUp
        );
        assert_eq!(
            tracker
                .update(start + Duration::from_secs(6), interfaces(), 10, 10)
                .1,
            MetricStatus::WarmingUp
        );
        assert_eq!(
            tracker
                .update(
                    start + Duration::from_secs(7),
                    ["wlan0".to_owned()].into(),
                    10,
                    10
                )
                .1,
            MetricStatus::WarmingUp
        );
        let (rate, state) = tracker.update(
            start + Duration::from_secs(8),
            ["wlan0".to_owned()].into(),
            1024,
            2048,
        );
        assert_eq!(state, MetricStatus::Available);
        let (sent, received) = rate.unwrap();
        assert!((sent - 2.0).abs() < 0.001);
        assert!((received - 1.0).abs() < 0.001);
    }

    #[test]
    fn network_rate_is_unavailable_when_no_interfaces_exist() {
        let mut tracker = NetworkRateTracker::default();
        let (rate, status) = tracker.update(Instant::now(), HashSet::new(), 0, 0);
        assert_eq!(rate, None);
        assert_eq!(status, MetricStatus::Unavailable);
    }
}
