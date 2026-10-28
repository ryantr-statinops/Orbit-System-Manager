use super::model::{GpuPreview, HistoryPoint};

const VRAM_TOTAL_GIB: f32 = 8.0;

pub fn generate_gpu(elapsed_seconds: u64) -> GpuPreview {
    let time = elapsed_seconds as f32;
    let history = (0..=60)
        .rev()
        .map(|seconds_ago| HistoryPoint {
            seconds_ago,
            value: Some(utilization_at(time - seconds_ago as f32)),
        })
        .collect();
    GpuPreview {
        name: "Demo GPU".into(),
        utilization: utilization_at(time),
        vram_used_gib: (4.1 + (time * 0.04).sin() * 0.7).clamp(0.0, VRAM_TOTAL_GIB),
        vram_total_gib: VRAM_TOTAL_GIB,
        temperature_c: 54.0 + (time * 0.06).sin() * 7.0,
        clock_mhz: 1485.0 + (time * 0.09).sin() * 180.0,
        history,
    }
}

fn utilization_at(time: f32) -> f32 {
    (31.0 + (time * 0.18).sin() * 19.0 + (time * 0.047).sin() * 12.0).clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::generate_gpu;

    #[test]
    fn gpu_preview_has_sixty_one_bounded_samples_and_hardware_values() {
        let gpu = generate_gpu(42);
        assert_eq!(gpu.history.len(), 61);
        assert_eq!(gpu.history.last().unwrap().value, Some(gpu.utilization));
        assert!((0.0..=100.0).contains(&gpu.utilization));
        assert!((0.0..=gpu.vram_total_gib).contains(&gpu.vram_used_gib));
        assert!(gpu.temperature_c.is_finite());
        assert!(gpu.clock_mhz.is_finite());
    }
}
