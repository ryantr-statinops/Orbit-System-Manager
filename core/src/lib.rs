use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandChild;

struct SidecarChild(Mutex<Option<CommandChild>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Start the Python backend sidecar (non-fatal if it fails)
      let child = app.shell()
        .sidecar("app")
        .ok()
        .and_then(|cmd| {
          cmd.args(["--port", "8080", "--quiet"])
             .spawn()
             .ok()
        })
        .map(|(_rx, child)| child);

      if child.is_some() {
        log::info!("Python backend sidecar started on port 8080");
      } else {
        log::warn!("Python backend sidecar not available - frontend will use mock telemetry");
      }

      app.manage(SidecarChild(Mutex::new(child)));
      Ok(())
    })
    .on_window_event(|window, event| {
      if let tauri::WindowEvent::CloseRequested { .. } = event {
        if let Some(state) = window.try_state::<SidecarChild>() {
          if let Ok(mut guard) = state.0.lock() {
            if let Some(child) = guard.take() {
              let _ = child.kill();
              log::info!("Python backend sidecar stopped");
            }
          }
        }
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
