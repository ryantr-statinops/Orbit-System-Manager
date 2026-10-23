// Orbit System Manager — Pure Native Rust & Slint Entry Point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use std::rc::Rc;
use std::time::Duration;
use slint::{Timer, TimerMode, VecModel};

pub mod telemetry;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();

    // Initialize the hardware telemetry collector
    let mut collector = telemetry::collector::TelemetryCollector::new();

    // 1 Hz repeated timer tied to Slint event loop
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_millis(1000), move || {
        if let Some(app) = app_weak.upgrade() {
            let payload = collector.collect();

            // 1. CPU metrics
            app.set_cpu_avg(payload.cpu_detail.avg_utilization);
            app.set_cpu_std(payload.cpu_detail.std_dev);
            app.set_cpu_var(payload.cpu_detail.variance);

            let core_items: Vec<CoreItem> = payload
                .cpu_detail
                .cores
                .iter()
                .enumerate()
                .map(|(idx, &load)| CoreItem {
                    id: idx as i32,
                    load,
                })
                .collect();
            let cores_model = Rc::new(VecModel::from(core_items));
            app.set_cores(cores_model.into());

            // 2. Memory metrics
            app.set_ram_percent(payload.memory_detail.percent as f32);
            app.set_ram_used_gb(payload.memory_detail.used_gb as f32);
            app.set_ram_total_gb(payload.memory_detail.total_gb as f32);

            // 3. Network speeds
            app.set_net_sent_kbps(payload.network_speed.sent_kbps as f32);
            app.set_net_recv_kbps(payload.network_speed.received_kbps as f32);

            // 4. GPU telemetry
            if let Some(gpu) = payload.gpu_detail.first() {
                app.set_gpu_load(gpu.load_percent);
                app.set_gpu_temp(gpu.temperature_c);
                app.set_gpu_name(slint::SharedString::from(gpu.name.clone()));
            }
        }
    });

    app.run()
}
