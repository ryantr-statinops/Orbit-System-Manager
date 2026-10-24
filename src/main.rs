// Orbit System Manager — Pure Native Rust & Slint Main Application
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use slint::{ModelRc, Timer, TimerMode, VecModel};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub mod renderer;
pub mod telemetry;

use renderer::OffscreenRenderer;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();
    let core_columns = Rc::new(Cell::new(8usize));

    {
        let columns = core_columns.clone();
        app.on_resize_core_grid(move |width| {
            columns.set(columns_for_width(width));
        });
    }

    // 1. Initialize hardware telemetry collector
    let latest_payload = Arc::new(Mutex::new(None));
    let stop_worker = Arc::new(AtomicBool::new(false));
    let worker_payload = latest_payload.clone();
    let worker_stop = stop_worker.clone();
    let telemetry_worker = thread::spawn(move || {
        let mut collector = telemetry::collector::TelemetryCollector::new();
        thread::sleep(Duration::from_secs(1));
        while !worker_stop.load(Ordering::Relaxed) {
            let started = Instant::now();
            let payload = collector.collect();
            if let Ok(mut latest) = worker_payload.lock() {
                *latest = Some(payload);
            }
            let remaining = Duration::from_secs(1).saturating_sub(started.elapsed());
            thread::sleep(remaining);
        }
    });

    // 2. Initialize native 3D renderer (640x380 viewport resolution)
    let renderer = Rc::new(RefCell::new(OffscreenRenderer::new(640, 380)));
    let frame_dirty = Rc::new(Cell::new(false));
    let animation_timer = Rc::new(Timer::default());

    // Camera callbacks only update state. The animation timer coalesces redraws.
    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_rotate_3d(move |dx, dy| {
            renderer_ref.borrow_mut().camera.rotate(dx, dy);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_pan_3d(move |dx, dy| {
            renderer_ref.borrow_mut().camera.pan(dx, dy);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_zoom_3d(move |delta| {
            renderer_ref.borrow_mut().camera.zoom(delta);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_resize_viewport(move |width, height| {
            renderer_ref
                .borrow_mut()
                .resize(width.round() as u32, height.round() as u32);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_reset_3d_view(move || {
            renderer_ref.borrow_mut().camera.reset();
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                timer.clone(),
            );
        });
    }

    // 4. Initial 3D frame render
    {
        let frame = renderer.borrow_mut().render_frame();
        app.set_viewport_3d_frame(frame);
        app.set_has_3d_frame(true);
        frame_dirty.set(false);
    }

    // Poll the latest worker snapshot on the UI thread.
    let telemetry_timer = Timer::default();
    {
        let app_weak_ref = app_weak.clone();
        let payload_ref = latest_payload.clone();
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let animation_timer = animation_timer.clone();
        let core_columns = core_columns.clone();

        telemetry_timer.start(TimerMode::Repeated, Duration::from_millis(100), move || {
            if let Some(app) = app_weak_ref.upgrade() {
                let payload = payload_ref.lock().ok().and_then(|mut p| p.take());
                let Some(payload) = payload else {
                    return;
                };

                // CPU metrics
                let cpu = &payload.cpu_detail;
                app.set_cpu_available(cpu.status == telemetry::models::MetricStatus::Available);
                let cpu_status_text = match cpu.status {
                    telemetry::models::MetricStatus::Available => "Available",
                    telemetry::models::MetricStatus::WarmingUp => "Collecting first sample",
                    telemetry::models::MetricStatus::Unavailable => "CPU telemetry unavailable",
                };
                app.set_cpu_status_text(cpu_status_text.into());
                app.set_cpu_avg(cpu.avg_utilization.unwrap_or_default());
                app.set_cpu_std(cpu.std_dev.unwrap_or_default());
                app.set_cpu_var(cpu.variance.unwrap_or_default());
                app.set_cpu_name(cpu.name.clone().unwrap_or_else(|| "N/A".into()).into());
                app.set_cpu_frequency(
                    cpu.frequency_ghz
                        .map(|f| format!("{f:.2} GHz"))
                        .unwrap_or_else(|| "N/A".into())
                        .into(),
                );
                app.set_cpu_physical_cores(
                    cpu.physical_core_count
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| "N/A".into())
                        .into(),
                );
                app.set_cpu_logical_cores(cpu.cores.len() as i32);

                let rows = build_core_rows(&cpu.cores, core_columns.get());
                app.set_core_rows(Rc::new(VecModel::from(rows)).into());

                // Push cores to 3D terrain heightmap
                renderer_ref.borrow_mut().mesh.push_sample(&cpu.cores);
                dirty.set(true);

                // Memory metrics
                app.set_ram_available(payload.memory_detail.percent.is_some());
                app.set_ram_percent(payload.memory_detail.percent.unwrap_or_default() as f32);
                app.set_ram_used_gb(payload.memory_detail.used_gib as f32);
                app.set_ram_total_gb(payload.memory_detail.total_gib as f32);

                // Network speeds
                let network = &payload.network_speed;
                app.set_network_available(
                    network.status == telemetry::models::MetricStatus::Available,
                );
                let network_status_text = match network.status {
                    telemetry::models::MetricStatus::Available => "Available",
                    telemetry::models::MetricStatus::WarmingUp => "Collecting first sample",
                    telemetry::models::MetricStatus::Unavailable => "No network interface",
                };
                app.set_network_status_text(network_status_text.into());
                app.set_net_sent_kib(network.sent_kib_per_sec.unwrap_or_default() as f32);
                app.set_net_recv_kib(network.received_kib_per_sec.unwrap_or_default() as f32);
                let ren = renderer_ref.borrow();
                app.set_history_samples(ren.mesh.sample_count() as i32);
                app.set_history_seconds(ren.mesh.history_seconds() as i32);
                drop(ren);
                schedule_animation(
                    app_weak_ref.clone(),
                    renderer_ref.clone(),
                    dirty.clone(),
                    animation_timer.clone(),
                );
            }
        });
    }

    let result = app.run();
    stop_worker.store(true, Ordering::Relaxed);
    let _ = telemetry_worker.join();
    result
}

fn columns_for_width(width: f32) -> usize {
    ((width - 48.0) / 108.0).floor().clamp(2.0, 8.0) as usize
}

fn build_core_rows(cores: &[f32], columns: usize) -> Vec<CoreRow> {
    let columns = columns.max(1);
    cores
        .chunks(columns)
        .enumerate()
        .map(|(row, values)| CoreRow {
            cells: ModelRc::from(Rc::new(VecModel::from(
                values
                    .iter()
                    .enumerate()
                    .map(|(column, &load)| CoreItem {
                        id: (row * columns + column) as i32,
                        load,
                    })
                    .collect::<Vec<_>>(),
            ))),
        })
        .collect()
}

fn schedule_animation(
    app_weak: slint::Weak<AppWindow>,
    renderer: Rc<RefCell<OffscreenRenderer>>,
    dirty: Rc<Cell<bool>>,
    timer: Rc<Timer>,
) {
    let callback_timer = timer.clone();
    timer.start(
        TimerMode::SingleShot,
        Duration::from_millis(33),
        move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            let mut ren = renderer.borrow_mut();
            let moving = ren.mesh.update_interpolation(0.12);
            if moving || dirty.replace(false) {
                app.set_viewport_3d_frame(ren.render_frame());
                app.set_has_3d_frame(true);
            }
            drop(ren);
            if moving {
                schedule_animation(
                    app_weak.clone(),
                    renderer.clone(),
                    dirty.clone(),
                    callback_timer.clone(),
                );
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::{build_core_rows, columns_for_width};
    use slint::Model;

    #[test]
    fn processor_columns_adapt_to_window_width() {
        assert_eq!(columns_for_width(720.0), 6);
        assert_eq!(columns_for_width(1280.0), 8);
        assert_eq!(columns_for_width(1600.0), 8);
    }

    #[test]
    fn processor_rows_cover_all_supported_core_counts() {
        for count in [0, 1, 4, 16, 32, 64] {
            let cores = vec![42.0; count];
            let rows = build_core_rows(&cores, 6);
            assert_eq!(
                rows.iter().map(|row| row.cells.row_count()).sum::<usize>(),
                count
            );
        }
    }
}
