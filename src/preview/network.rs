use super::model::{HistoryPoint, NetworkPreview};

pub fn generate_network(elapsed_seconds: u64) -> NetworkPreview {
    let time = elapsed_seconds as f32;
    let (received_kib_per_sec, sent_kib_per_sec) = rates_at(time);
    let received_history = (0..=60)
        .rev()
        .map(|seconds_ago| HistoryPoint {
            seconds_ago,
            value: Some(rates_at(time - seconds_ago as f32).0),
        })
        .collect();
    let sent_history = (0..=60)
        .rev()
        .map(|seconds_ago| HistoryPoint {
            seconds_ago,
            value: Some(rates_at(time - seconds_ago as f32).1),
        })
        .collect();
    NetworkPreview {
        received_kib_per_sec,
        sent_kib_per_sec,
        received_history,
        sent_history,
    }
}

fn rates_at(time: f32) -> (f32, f32) {
    let receive = (420.0 + (time * 0.27).sin() * 160.0 + (time * 0.071).sin() * 95.0).max(0.0);
    let send = (105.0 + (time * 0.19 + 0.8).sin() * 62.0 + (time * 0.083).sin() * 31.0).max(0.0);
    (receive, send)
}

#[cfg(test)]
mod tests {
    use super::generate_network;

    #[test]
    fn network_preview_tracks_both_directions_over_the_same_window() {
        let network = generate_network(42);
        assert_eq!(network.received_history.len(), 61);
        assert_eq!(network.sent_history.len(), 61);
        assert_eq!(
            network.received_history.last().unwrap().value,
            Some(network.received_kib_per_sec)
        );
        assert_eq!(
            network.sent_history.last().unwrap().value,
            Some(network.sent_kib_per_sec)
        );
        assert!(network.received_kib_per_sec >= network.sent_kib_per_sec);
    }
}
