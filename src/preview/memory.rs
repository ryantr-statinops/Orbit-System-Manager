use super::model::{HistoryPoint, MemoryPreview};

const TOTAL_GIB: f32 = 31.8;

pub fn generate_memory(elapsed_seconds: u64) -> MemoryPreview {
    let used_gib = memory_at(elapsed_seconds as f32);
    let history = (0..=60)
        .rev()
        .map(|seconds_ago| HistoryPoint {
            seconds_ago,
            value: Some(memory_at(elapsed_seconds as f32 - seconds_ago as f32)),
        })
        .collect();
    MemoryPreview {
        used_gib,
        total_gib: TOTAL_GIB,
        history,
    }
}

fn memory_at(time: f32) -> f32 {
    (12.2 + (time * 0.043).sin() * 1.25 + (time * 0.13).sin() * 0.32)
        .clamp(0.0, TOTAL_GIB)
}

#[cfg(test)]
mod tests {
    use super::generate_memory;

    #[test]
    fn memory_preview_has_consistent_capacity_and_bounded_values() {
        let memory = generate_memory(60);
        assert_eq!(memory.history.len(), 61);
        assert_eq!(memory.history.last().unwrap().value, Some(memory.used_gib));
        assert!(memory.history.iter().all(|point| {
            point.value.is_some_and(|value| (0.0..=memory.total_gib).contains(&value))
        }));
    }

    #[test]
    fn memory_preview_is_repeatable() {
        assert_eq!(generate_memory(7), generate_memory(7));
    }
}
