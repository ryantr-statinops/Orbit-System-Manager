// Orbit System Manager — Pure Native Rust & Slint Main Application
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use slint::{Timer, TimerMode, VecModel};

pub mod renderer;
pub mod telemetry;

use renderer::OffscreenRenderer;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();

    // 1. Initialize hardware telemetry collector
    let collector = Rc::new(RefCell::new(telemetry::collector::TelemetryCollector::new()));

    // 2. Initialize native 3D renderer (640x380 viewport resolution)
    let renderer = Rc::new(RefCell::new(OffscreenRenderer::new(640, 380)));

    // 3. Connect 3D Interactive Mouse Controls
    {
        let renderer_ref = renderer.clone();
        let app_weak_ref = app_weak.clone();
        app.on_rotate_3d(move |dx, dy| {
            renderer_ref.borrow_mut().camera.rotate(dx, dy);
            if let Some(app) = app_weak_ref.upgrade() {
                let frame = renderer_ref.borrow_mut().render_frame();
                app.set_viewport_3d_frame(frame);
            }
        });
    }

    {
        let renderer_ref = renderer.clone();
        let app_weak_ref = app_weak.clone();
        app.on_pan_3d(move |dx, dy| {
            renderer_ref.borrow_mut().camera.pan(dx, dy);
            if let Some(app) = app_weak_ref.upgrade() {
                let frame = renderer_ref.borrow_mut().render_frame();
                app.set_viewport_3d_frame(frame);
            }
        });
    }

    {
        let renderer_ref = renderer.clone();
        let app_weak_ref = app_weak.clone();
        app.on_zoom_3d(move |delta| {
            renderer_ref.borrow_mut().camera.zoom(delta);
            if let Some(app) = app_weak_ref.upgrade() {
                let frame = renderer_ref.borrow_mut().render_frame();
                app.set_viewport_3d_frame(frame);
            }
        });
    }

    // 4. Initial 3D frame render
    {
        let frame = renderer.borrow_mut().render_frame();
        app.set_viewport_3d_frame(frame);
        app.set_has_3d_frame(true);
    }

    // 5. Telemetry update timer (1 Hz interval)
    let telemetry_timer = Timer::default();
    {
        let app_weak_ref = app_weak.clone();
        let collector_ref = collector.clone();
        let renderer_ref = renderer.clone();

        telemetry_timer.start(TimerMode::Repeated, Duration::from_millis(1000), move || {
            if let Some(app) = app_weak_ref.upgrade() {
                let payload = collector_ref.borrow_mut().collect();

                // CPU metrics
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

                // Push cores to 3D terrain heightmap
                renderer_ref
                    .borrow_mut()
                    .mesh
                    .push_sample(&payload.cpu_detail.cores);

                // Memory metrics
                app.set_ram_percent(payload.memory_detail.percent as f32);
                app.set_ram_used_gb(payload.memory_detail.used_gb as f32);
                app.set_ram_total_gb(payload.memory_detail.total_gb as f32);

                // Network speeds
                app.set_net_sent_kbps(payload.network_speed.sent_kbps as f32);
                app.set_net_recv_kbps(payload.network_speed.received_kbps as f32);

                // GPU telemetry
                if let Some(gpu) = payload.gpu_detail.first() {
                    app.set_gpu_load(gpu.load_percent);
                    app.set_gpu_temp(gpu.temperature_c);
                    app.set_gpu_name(slint::SharedString::from(gpu.name.clone()));
                }
            }
        });
    }

    // 6. Smooth 3D animation interpolation loop (~30 FPS)
    let animation_timer = Timer::default();
    {
        let app_weak_ref = app_weak.clone();
        let renderer_ref = renderer.clone();

        animation_timer.start(TimerMode::Repeated, Duration::from_millis(33), move || {
            if let Some(app) = app_weak_ref.upgrade() {
                let mut ren = renderer_ref.borrow_mut();
                ren.mesh.update_interpolation(0.12);
                let frame = ren.render_frame();
                app.set_viewport_3d_frame(frame);
                app.set_has_3d_frame(true);
            }
        });
    }

    app.run()
}
