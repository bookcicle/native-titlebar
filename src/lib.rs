use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

mod commands;
mod error;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::NativeTitlebar;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the native-titlebar APIs.
pub trait NativeTitlebarExt<R: Runtime> {
  fn native_titlebar(&self) -> &NativeTitlebar<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NativeTitlebarExt<R> for T {
  fn native_titlebar(&self) -> &NativeTitlebar<R> {
    self.state::<NativeTitlebar<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("native-titlebar")
    .invoke_handler(tauri::generate_handler![commands::titlebar])
    .setup(|app, api| {
      #[cfg(desktop)]
      let native_titlebar = desktop::init(app, api)?;
      app.manage(native_titlebar);
      Ok(())
    })
    .build()
}
