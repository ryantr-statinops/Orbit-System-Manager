pub mod telemetry;

use std::time::Duration;
use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Start native Rust telemetry background thread (1 Hz)
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let mut collector = telemetry::collector::TelemetryCollector::new();
                log::info!("Native Rust Telemetry Collector started (1 Hz interval)");

                loop {
                    std::thread::sleep(Duration::from_secs(1));
                    let payload = collector.collect();
                    if let Err(err) = app_handle.emit("telemetry-data", &payload) {
                        log::debug!("Telemetry emit error: {err}");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
