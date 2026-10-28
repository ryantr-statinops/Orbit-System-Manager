use super::model::{CpuPreview, HistoryPoint, ProcessorPreview};

pub const HISTORY_SECONDS: u8 = 60;

pub fn generate_cpu(elapsed_seconds: u64, core_count: usize) -> CpuPreview {
    let cores = (0..core_count)
        .map(|id| ProcessorPreview {
            id,
            utilization: core_load(id, elapsed_seconds as f32),
            history: history(|seconds| core_load(id, seconds)),
        })
        .collect::<Vec<_>>();

    let values = cores.iter().map(|core| core.utilization).collect::<Vec<_>>();
    let average = (!values.is_empty()).then(|| values.iter().sum::<f32>() / values.len() as f32);
    let variance = average.map(|mean| {
        values.iter().map(|value| (value - mean).powi(2)).sum::<f32>() / values.len() as f32
    });

    let cpu_history = history(|seconds| {
        let values = (0..core_count)
            .map(|id| core_load(id, seconds))
            .collect::<Vec<_>>();
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f32>() / values.len() as f32
        }
    });

    CpuPreview {
        name: format!("Demo CPU ({core_count} logical processors)"),
        frequency_ghz: (core_count > 0).then_some(3.6),
        physical_cores: (core_count > 0).then_some(core_count.div_ceil(2)),
        cores,
        average,
        std_dev: variance.map(f32::sqrt),
        variance,
        history: cpu_history,
    }
}

fn history(mut value_at: impl FnMut(f32) -> f32) -> Vec<HistoryPoint> {
    (0..=HISTORY_SECONDS)
        .rev()
        .map(|seconds_ago| HistoryPoint {
            seconds_ago,
            value: Some(value_at(-(seconds_ago as f32))),
        })
        .collect()
}

fn core_load(id: usize, time: f32) -> f32 {
    let phase = id as f32 * 0.53;
    let slow = (time * 0.115 + phase).sin() * 22.0;
    let fast = (time * 0.31 + phase * 1.7).sin() * 11.0;
    let pulse = if (time as i64 + id as i64 * 7).rem_euclid(47) < 4 {
        30.0
    } else {
        0.0
    };
    (34.0 + slow + fast + pulse).clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::generate_cpu;

    #[test]
    fn cpu_preview_is_deterministic_and_has_sixty_seconds_of_history() {
        let first = generate_cpu(60, 16);
        let second = generate_cpu(60, 16);
        assert_eq!(first, second);
        assert_eq!(first.history.len(), 61);
        assert_eq!(first.history.first().unwrap().seconds_ago, 60);
        assert_eq!(first.history.last().unwrap().seconds_ago, 0);
    }

    #[test]
    fn cpu_statistics_are_derived_from_the_current_processor_values() {
        let cpu = generate_cpu(18, 16);
        let mean = cpu.cores.iter().map(|core| core.utilization).sum::<f32>() / 16.0;
        let variance = cpu
            .cores
            .iter()
            .map(|core| (core.utilization - mean).powi(2))
            .sum::<f32>()
            / 16.0;
        assert!((cpu.average.unwrap() - mean).abs() < 0.001);
        assert!((cpu.variance.unwrap() - variance).abs() < 0.001);
        assert!((cpu.std_dev.unwrap() - variance.sqrt()).abs() < 0.001);
    }

    #[test]
    fn cpu_preview_supports_zero_and_one_core() {
        let empty = generate_cpu(0, 0);
        assert!(empty.cores.is_empty());
        assert_eq!(empty.average, None);
        let single = generate_cpu(0, 1);
        assert_eq!(single.cores.len(), 1);
        assert!(single.average.is_some());
    }
}
