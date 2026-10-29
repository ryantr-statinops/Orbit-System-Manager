// Orbit System Manager — Pure Native Rust & Slint Main Application
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;

use orbit_system_manager::preview::DashboardSnapshot;
use orbit_system_manager::ui::{AppWindow, CoreItem, CoreRow, LinePlotSeries, TerrainAxisLabel};
use slint::{ComponentHandle, ModelRc, Timer, TimerMode, VecModel};
use std::cell::Cell;
use std::cell::RefCell;
#[cfg(feature = "dashboard-screenshot")]
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use orbit_system_manager::{preview, renderer};
use renderer::OffscreenRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = cli::parse_demo_args(std::env::args().skip(1)).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    });

    #[cfg(not(feature = "dashboard-screenshot"))]
    if options.capture_path.is_some() {
        eprintln!("Screenshot capture requires building with --features dashboard-screenshot.");
        std::process::exit(2);
    }
    #[cfg(feature = "dashboard-screenshot")]
    if options.capture_path.is_some() {
        let backend = i_slint_backend_testing::TestingBackend::new(
            i_slint_backend_testing::TestingBackendOptions {
                mock_time: false,
                threading: true,
                renderer_name: Some("software".into()),
            },
        );
        slint::platform::set_platform(Box::new(backend))
            .expect("could not initialize Slint screenshot backend");
    }

    let app = AppWindow::new()?;
    if options.capture_path.is_some() {
        app.window().set_size(slint::LogicalSize::new(
            options.capture_size.0 as f32,
            options.capture_size.1 as f32,
        ));
    }
    let app_weak = app.as_weak();
    let core_columns = Rc::new(Cell::new(8usize));
    let demo_source = Rc::new(RefCell::new(preview::PreviewState::with_scenario(
        options.core_count,
        options.scenario,
    )));
    let snapshot = demo_source.borrow().snapshot();

    {
        let columns = core_columns.clone();
        app.on_resize_core_grid(move |width| {
            columns.set(columns_for_width(width));
        });
    }

    // Initialize the native 3D renderer with the initial demo sample.
    let renderer = Rc::new(RefCell::new(OffscreenRenderer::new(640, 380)));
    #[cfg(feature = "dashboard-screenshot")]
    if options.capture_path.is_some() {
        let (width, height) = capture_canvas_size(options.capture_size);
        renderer
            .borrow_mut()
            .resize_scaled(width as f32, height as f32, 1.0);
    }
    let frame_dirty = Rc::new(Cell::new(false));
    let viewport_active = Rc::new(Cell::new(true));
    let animation_timer = Rc::new(Timer::default());

    apply_snapshot(&app, &snapshot, core_columns.get());
    seed_terrain_history(&mut renderer.borrow_mut(), &snapshot.cores);
    renderer.borrow_mut().fit_camera_to_viewport();
    {
        let mesh = renderer.borrow();
        app.set_history_samples(mesh.mesh.sample_count() as i32);
        app.set_history_seconds(mesh.mesh.history_seconds() as i32);
    }

    {
        let source = demo_source.clone();
        let app_weak = app_weak.clone();
        app.on_toggle_demo_pause(move || {
            let paused = source.borrow_mut().toggle_pause();
            if let Some(app) = app_weak.upgrade() {
                app.set_demo_paused(paused);
            }
        });
    }

    // Camera callbacks only update state. The animation timer coalesces redraws.
    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let active = viewport_active.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_rotate_3d(move |dx, dy| {
            renderer_ref.borrow_mut().camera.rotate(dx, dy);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                active.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let active = viewport_active.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_pan_3d(move |dx, dy| {
            renderer_ref.borrow_mut().camera.pan(dx, dy);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                active.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let active = viewport_active.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_zoom_3d(move |delta| {
            renderer_ref.borrow_mut().camera.zoom(delta);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                active.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let active = viewport_active.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_resize_viewport(move |width, height| {
            if width <= 0.0 || height <= 0.0 {
                active.set(false);
                dirty.set(true);
                timer.stop();
                return;
            }
            let scale_factor = app_weak
                .upgrade()
                .map(|app| app.window().scale_factor())
                .unwrap_or(1.0);
            if !renderer_ref
                .borrow_mut()
                .resize_scaled(width, height, scale_factor)
            {
                active.set(false);
                dirty.set(true);
                timer.stop();
                return;
            }
            active.set(true);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                active.clone(),
                timer.clone(),
            );
        });
    }

    {
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let active = viewport_active.clone();
        let timer = animation_timer.clone();
        let app_weak = app_weak.clone();
        app.on_reset_3d_view(move || {
            let mut renderer = renderer_ref.borrow_mut();
            renderer.camera.reset();
            renderer.fit_camera_to_viewport();
            drop(renderer);
            dirty.set(true);
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                active.clone(),
                timer.clone(),
            );
        });
    }

    // 4. Initial 3D frame render
    {
        let mut renderer = renderer.borrow_mut();
        let frame = renderer.render_frame();
        app.set_viewport_3d_frame(frame);
        app.set_has_3d_frame(true);
        apply_terrain_axis_labels(&app, &renderer);
        frame_dirty.set(false);
    }

    #[cfg(feature = "dashboard-screenshot")]
    if let Some(path) = options.capture_path.as_deref() {
        return capture_dashboard(&app, app_weak, path);
    }

    let demo_timer = Timer::default();
    {
        let source = demo_source.clone();
        let app_weak = app_weak.clone();
        let columns = core_columns.clone();
        let renderer_ref = renderer.clone();
        let dirty = frame_dirty.clone();
        let active = viewport_active.clone();
        let animation = animation_timer.clone();
        demo_timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            let Some(snapshot) = source.borrow_mut().tick() else {
                return;
            };
            let core_values = apply_snapshot(&app, &snapshot, columns.get());
            {
                let mut ren = renderer_ref.borrow_mut();
                if !core_values.is_empty() {
                    ren.mesh.push_sample(&core_values);
                }
                dirty.set(true);
            }
            {
                let ren = renderer_ref.borrow();
                app.set_history_samples(ren.mesh.sample_count() as i32);
                app.set_history_seconds(ren.mesh.history_seconds() as i32);
            }
            schedule_animation(
                app_weak.clone(),
                renderer_ref.clone(),
                dirty.clone(),
                active.clone(),
                animation.clone(),
            );
        });
    }

    app.run()
}

#[cfg(feature = "dashboard-screenshot")]
fn capture_dashboard(
    app: &AppWindow,
    app_weak: slint::Weak<AppWindow>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let capture_result = Rc::new(RefCell::new(None::<Result<(), String>>));
    let capture_result_ref = capture_result.clone();
    let path = path.to_owned();
    let capture_timer = Timer::default();
    capture_timer.start(
        TimerMode::SingleShot,
        Duration::from_millis(150),
        move || {
            let result = app_weak
                .upgrade()
                .ok_or_else(|| "dashboard closed before screenshot capture".to_string())
                .and_then(|app| save_dashboard_screenshot(&app, &path));
            *capture_result_ref.borrow_mut() = Some(result);
            let _ = slint::quit_event_loop();
        },
    );
    app.window().show()?;
    app.run()?;
    capture_result
        .borrow_mut()
        .take()
        .unwrap_or_else(|| Err("screenshot timer did not run".into()))?;
    Ok(())
}

#[cfg(feature = "dashboard-screenshot")]
fn save_dashboard_screenshot(app: &AppWindow, path: &Path) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    let pixels = app
        .window()
        .take_snapshot()
        .map_err(|error| format!("could not capture Slint window: {error}"))?;
    let image =
        image::RgbaImage::from_raw(pixels.width(), pixels.height(), pixels.as_bytes().to_vec())
            .ok_or_else(|| "Slint returned an invalid RGBA screenshot buffer".to_owned())?;
    image
        .save(path)
        .map_err(|error| format!("could not save screenshot to {}: {error}", path.display()))
}

fn columns_for_width(width: f32) -> usize {
    if width >= 1060.0 {
        1
    } else {
        ((width - 48.0) / 148.0).floor().clamp(1.0, 8.0) as usize
    }
}

#[cfg(feature = "dashboard-screenshot")]
fn capture_canvas_size(window_size: (u32, u32)) -> (u32, u32) {
    let (window_width, window_height) = window_size;
    if window_width >= 1060 {
        (
            window_width.saturating_sub(630).max(1),
            window_height.saturating_sub(202).max(1),
        )
    } else {
        (window_width.saturating_sub(40).max(1), 330)
    }
}

fn apply_snapshot(app: &AppWindow, snapshot: &DashboardSnapshot, columns: usize) -> Vec<f32> {
    let core_values = snapshot
        .cores
        .iter()
        .map(|core| core.utilization)
        .collect::<Vec<_>>();

    let status_text = match snapshot.scenario {
        preview::DemoScenario::Full | preview::DemoScenario::Single => "Demo data",
        preview::DemoScenario::Empty => "Waiting for samples",
        preview::DemoScenario::Unavailable => "Unavailable",
    };
    app.set_cpu_available(snapshot.cpu_average.is_some());
    let cpu_status_text =
        if snapshot.scenario == preview::DemoScenario::Full && snapshot.cores.is_empty() {
            "No logical processors"
        } else {
            status_text
        };
    app.set_cpu_status_text(cpu_status_text.into());
    app.set_cpu_avg(snapshot.cpu_average.unwrap_or_default());
    app.set_cpu_std(snapshot.cpu_std_dev.unwrap_or_default());
    app.set_cpu_var(snapshot.cpu_variance.unwrap_or_default());
    let cpu_history_points =
        preview::normalize_history(&snapshot.cpu_history, preview::HISTORY_SECONDS, 100.0);
    let cpu_history_has_data = cpu_history_points.iter().any(|point| point.y.is_some());
    let cpu_history_line = preview::line_path_commands(&cpu_history_points);
    app.set_cpu_history_area_commands(preview::area_path_commands(&cpu_history_points).into());
    app.set_cpu_history_series(
        Rc::new(VecModel::from(if cpu_history_line.is_empty() {
            Vec::new()
        } else {
            vec![LinePlotSeries {
                commands: cpu_history_line.into(),
                color: slint::Color::from_rgb_u8(99, 214, 232),
            }]
        }))
        .into(),
    );
    app.set_cpu_history_has_data(cpu_history_has_data);
    let cpu_name = match snapshot.scenario {
        preview::DemoScenario::Full | preview::DemoScenario::Single => snapshot.cpu_name.as_str(),
        preview::DemoScenario::Empty | preview::DemoScenario::Unavailable => status_text,
    };
    app.set_cpu_name(cpu_name.into());
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
    app.set_core_rows(Rc::new(VecModel::from(build_core_rows(&snapshot.cores, columns))).into());

    let ram_available = snapshot.ram_used_gib.zip(snapshot.ram_total_gib);
    app.set_ram_available(ram_available.is_some());
    app.set_ram_status_text(status_text.into());
    app.set_ram_used_gb(snapshot.ram_used_gib.unwrap_or_default());
    app.set_ram_total_gb(snapshot.ram_total_gib.unwrap_or_default());
    app.set_ram_percent(
        ram_available
            .filter(|(_, total)| *total > 0.0)
            .map(|(used, total)| used / total * 100.0)
            .unwrap_or_default(),
    );
    let ram_history_points = preview::normalize_history(
        &snapshot.ram_history,
        preview::HISTORY_SECONDS,
        snapshot.ram_total_gib.unwrap_or_default(),
    );
    let ram_history_has_data = ram_available.is_some_and(|(_, total)| total > 0.0)
        && ram_history_points.iter().any(|point| point.y.is_some());
    let ram_history_line = preview::line_path_commands(&ram_history_points);
    app.set_ram_history_area_commands(preview::area_path_commands(&ram_history_points).into());
    app.set_ram_history_series(
        Rc::new(VecModel::from(if ram_history_line.is_empty() {
            Vec::new()
        } else {
            vec![LinePlotSeries {
                commands: ram_history_line.into(),
                color: slint::Color::from_rgb_u8(83, 214, 194),
            }]
        }))
        .into(),
    );
    app.set_ram_history_has_data(ram_history_has_data);

    app.set_gpu_available(snapshot.gpu.is_some());
    app.set_gpu_status_text(status_text.into());
    let gpu_history = snapshot
        .gpu
        .as_ref()
        .map(|gpu| preview::normalize_history(&gpu.history, preview::HISTORY_SECONDS, 100.0))
        .unwrap_or_default();
    let gpu_history_has_data = gpu_history.iter().any(|point| point.y.is_some());
    let gpu_history_line = preview::line_path_commands(&gpu_history);
    app.set_gpu_history_area_commands(preview::area_path_commands(&gpu_history).into());
    app.set_gpu_history_series(
        Rc::new(VecModel::from(if gpu_history_line.is_empty() {
            Vec::new()
        } else {
            vec![LinePlotSeries {
                commands: gpu_history_line.into(),
                color: slint::Color::from_rgb_u8(99, 214, 232),
            }]
        }))
        .into(),
    );
    app.set_gpu_history_has_data(gpu_history_has_data);
    if let Some(gpu) = &snapshot.gpu {
        app.set_gpu_name(gpu.name.clone().into());
        app.set_gpu_utilization(gpu.utilization);
        app.set_gpu_vram_used_gib(gpu.vram_used_gib);
        app.set_gpu_vram_total_gib(gpu.vram_total_gib);
        app.set_gpu_temperature_c(gpu.temperature_c);
        app.set_gpu_clock_mhz(gpu.clock_mhz);
    } else {
        app.set_gpu_name(status_text.into());
        app.set_gpu_utilization(0.0);
        app.set_gpu_vram_used_gib(0.0);
        app.set_gpu_vram_total_gib(0.0);
        app.set_gpu_temperature_c(0.0);
        app.set_gpu_clock_mhz(0.0);
    }

    if let Some(network) = &snapshot.network {
        app.set_network_available(true);
        app.set_network_status_text("Demo data".into());
        app.set_net_recv_kib(network.received_kib_per_sec);
        app.set_net_sent_kib(network.sent_kib_per_sec);
        let maximum =
            preview::network_history_max_value(&network.received_history, &network.sent_history);
        let received = preview::line_path_commands(&preview::normalize_history(
            &network.received_history,
            preview::HISTORY_SECONDS,
            maximum,
        ));
        let sent = preview::line_path_commands(&preview::normalize_history(
            &network.sent_history,
            preview::HISTORY_SECONDS,
            maximum,
        ));
        let mut series = Vec::new();
        if !received.is_empty() {
            series.push(LinePlotSeries {
                commands: received.into(),
                color: slint::Color::from_rgb_u8(99, 214, 232),
            });
        }
        if !sent.is_empty() {
            series.push(LinePlotSeries {
                commands: sent.into(),
                color: slint::Color::from_rgb_u8(100, 215, 161),
            });
        }
        app.set_network_history_has_data(!series.is_empty());
        app.set_network_history_series(Rc::new(VecModel::from(series)).into());
    } else {
        app.set_network_available(false);
        app.set_network_status_text(status_text.into());
        app.set_net_recv_kib(0.0);
        app.set_net_sent_kib(0.0);
        app.set_network_history_has_data(false);
        app.set_network_history_series(Rc::new(VecModel::from(Vec::new())).into());
    }

    core_values
}

fn build_core_rows(cores: &[preview::ProcessorPreview], columns: usize) -> Vec<CoreRow> {
    let columns = columns.max(1);
    cores
        .chunks(columns)
        .map(|values| CoreRow {
            cells: ModelRc::from(Rc::new(VecModel::from(
                values
                    .iter()
                    .map(|core| CoreItem {
                        id: core.id as i32,
                        load: core.utilization,
                        sparkline: preview::line_path_commands(&preview::normalize_history(
                            &core.history,
                            preview::HISTORY_SECONDS,
                            100.0,
                        ))
                        .into(),
                    })
                    .collect::<Vec<_>>(),
            ))),
        })
        .collect()
}

fn seed_terrain_history(renderer: &mut OffscreenRenderer, cores: &[preview::ProcessorPreview]) {
    let Some(first_core) = cores.first() else {
        return;
    };
    let newest_sample_time = std::time::Instant::now();
    for (index, point) in first_core.history.iter().enumerate() {
        let Some(values) = cores
            .iter()
            .map(|core| core.history.get(index)?.value)
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        let sampled_at = newest_sample_time - Duration::from_secs(point.seconds_ago as u64);
        renderer.mesh.push_sample_at(&values, sampled_at);
    }
}

fn schedule_animation(
    app_weak: slint::Weak<AppWindow>,
    renderer: Rc<RefCell<OffscreenRenderer>>,
    dirty: Rc<Cell<bool>>,
    active: Rc<Cell<bool>>,
    timer: Rc<Timer>,
) {
    if !active.get() {
        dirty.set(true);
        timer.stop();
        return;
    }
    let callback_timer = timer.clone();
    let callback_active = active.clone();
    timer.start(
        TimerMode::SingleShot,
        Duration::from_millis(33),
        move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            if !callback_active.get() {
                dirty.set(true);
                return;
            }
            let mut ren = renderer.borrow_mut();
            let moving = ren.mesh.update_interpolation(0.12);
            if moving || dirty.replace(false) {
                let frame = ren.render_frame();
                app.set_viewport_3d_frame(frame);
                app.set_has_3d_frame(true);
                apply_terrain_axis_labels(&app, &ren);
            }
            drop(ren);
            if moving {
                schedule_animation(
                    app_weak.clone(),
                    renderer.clone(),
                    dirty.clone(),
                    callback_active.clone(),
                    callback_timer.clone(),
                );
            }
        },
    );
}

fn apply_terrain_axis_labels(app: &AppWindow, renderer: &OffscreenRenderer) {
    let (scale_x, scale_y) = renderer.label_scale();
    let labels = renderer
        .projected_axis_labels()
        .into_iter()
        .map(|label| TerrainAxisLabel {
            x: label.x as f32 / scale_x,
            y: label.y as f32 / scale_y,
            width: label.width as f32 / scale_x,
            text: label.text.into(),
            color: slint::Color::from_rgb_u8(label.color[0], label.color[1], label.color[2]),
            is_axis_title: label.is_axis_title,
        })
        .collect::<Vec<_>>();
    app.set_terrain_axis_labels(Rc::new(VecModel::from(labels)).into());
}

#[cfg(test)]
mod tests {
    use super::{build_core_rows, columns_for_width};
    use orbit_system_manager::preview;
    use slint::Model;

    #[test]
    fn processor_columns_adapt_to_window_width() {
        assert_eq!(columns_for_width(720.0), 4);
        assert_eq!(columns_for_width(1059.0), 6);
        assert_eq!(columns_for_width(1060.0), 1);
        assert_eq!(columns_for_width(1280.0), 1);
        assert_eq!(columns_for_width(1600.0), 1);
    }

    #[test]
    fn processor_rows_cover_all_supported_core_counts() {
        for count in [0, 1, 4, 16, 32, 64] {
            let cores = preview::generate_cpu(0, count).cores;
            let rows = build_core_rows(&cores, 6);
            assert_eq!(
                rows.iter().map(|row| row.cells.row_count()).sum::<usize>(),
                count
            );
        }
    }

    #[test]
    fn processor_rows_include_their_own_timestamped_sparkline() {
        let cores = preview::generate_cpu(12, 4).cores;
        let rows = build_core_rows(&cores, 2);
        let first = rows[0].cells.row_data(0).unwrap();
        assert_eq!(first.id, 0);
        assert!(first.sparkline.to_string().starts_with("M "));
    }

    #[cfg(feature = "dashboard-screenshot")]
    #[test]
    fn screenshot_canvas_dimensions_follow_the_dashboard_breakpoint() {
        assert_eq!(super::capture_canvas_size((1465, 955)), (835, 753));
        assert_eq!(super::capture_canvas_size((1280, 800)), (650, 598));
        assert_eq!(super::capture_canvas_size((1060, 800)), (430, 598));
        assert_eq!(super::capture_canvas_size((720, 520)), (680, 330));
    }
}
