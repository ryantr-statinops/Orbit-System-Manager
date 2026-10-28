// Orbit System Manager — Pure Native Rust & Slint Main Application
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use orbit_system_manager::preview::DashboardSnapshot;
use orbit_system_manager::ui::{AppWindow, CoreItem, CoreRow};
use slint::{ModelRc, Timer, TimerMode, VecModel};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use orbit_system_manager::{preview, renderer};
use renderer::OffscreenRenderer;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();
    let core_columns = Rc::new(Cell::new(8usize));
    let snapshot = preview::snapshot_at(0, 16);

    {
        let columns = core_columns.clone();
        app.on_resize_core_grid(move |width| {
            columns.set(columns_for_width(width));
        });
    }

    // Initialize the native 3D renderer with the initial demo sample.
    let renderer = Rc::new(RefCell::new(OffscreenRenderer::new(640, 380)));
    let frame_dirty = Rc::new(Cell::new(false));
    let animation_timer = Rc::new(Timer::default());

    let core_values = apply_snapshot(&app, &snapshot, core_columns.get());
    renderer.borrow_mut().mesh.push_sample(&core_values);
    {
        let mesh = renderer.borrow();
        app.set_history_samples(mesh.mesh.sample_count() as i32);
        app.set_history_seconds(mesh.mesh.history_seconds() as i32);
    }

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

    app.run()
}

fn columns_for_width(width: f32) -> usize {
    if width >= 1060.0 {
        1
    } else {
        ((width - 48.0) / 108.0).floor().clamp(1.0, 8.0) as usize
    }
}

fn apply_snapshot(app: &AppWindow, snapshot: &DashboardSnapshot, columns: usize) -> Vec<f32> {
    let core_values = snapshot
        .cores
        .iter()
        .map(|core| core.utilization)
        .collect::<Vec<_>>();

    app.set_cpu_available(snapshot.cpu_average.is_some());
    app.set_cpu_status_text("Demo data".into());
    app.set_cpu_avg(snapshot.cpu_average.unwrap_or_default());
    app.set_cpu_std(snapshot.cpu_std_dev.unwrap_or_default());
    app.set_cpu_var(snapshot.cpu_variance.unwrap_or_default());
    app.set_cpu_name(snapshot.cpu_name.clone().into());
    app.set_cpu_frequency(
        snapshot
            .cpu_frequency_ghz
            .map(|frequency| format!("{frequency:.2} GHz"))
            .unwrap_or_else(|| "N/A".into())
            .into(),
    );
    app.set_cpu_physical_cores(
        snapshot
            .physical_cores
            .map(|count| count.to_string())
            .unwrap_or_else(|| "N/A".into())
            .into(),
    );
    app.set_cpu_logical_cores(snapshot.cores.len() as i32);
    app.set_core_rows(Rc::new(VecModel::from(build_core_rows(&core_values, columns))).into());

    let ram_available = snapshot.ram_used_gib.zip(snapshot.ram_total_gib);
    app.set_ram_available(ram_available.is_some());
    app.set_ram_used_gb(snapshot.ram_used_gib.unwrap_or_default());
    app.set_ram_total_gb(snapshot.ram_total_gib.unwrap_or_default());
    app.set_ram_percent(
        ram_available
            .filter(|(_, total)| *total > 0.0)
            .map(|(used, total)| used / total * 100.0)
            .unwrap_or_default(),
    );

    if let Some(gpu) = &snapshot.gpu {
        app.set_gpu_name(gpu.name.clone().into());
        app.set_gpu_utilization(gpu.utilization);
        app.set_gpu_vram_used_gib(gpu.vram_used_gib);
        app.set_gpu_vram_total_gib(gpu.vram_total_gib);
        app.set_gpu_temperature_c(gpu.temperature_c);
        app.set_gpu_clock_mhz(gpu.clock_mhz);
    }

    if let Some(network) = &snapshot.network {
        app.set_network_available(true);
        app.set_network_status_text("Demo data".into());
        app.set_net_recv_kib(network.received_kib_per_sec);
        app.set_net_sent_kib(network.sent_kib_per_sec);
    }

    core_values
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
        assert_eq!(columns_for_width(1059.0), 8);
        assert_eq!(columns_for_width(1060.0), 1);
        assert_eq!(columns_for_width(1280.0), 1);
        assert_eq!(columns_for_width(1600.0), 1);
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
