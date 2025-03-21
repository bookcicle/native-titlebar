use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<NativeTitlebar<R>> {
  Ok(NativeTitlebar(app.clone()))
}

/// Access to the native-titlebar APIs.
pub struct NativeTitlebar<R: Runtime>(AppHandle<R>);

impl<R: Runtime> NativeTitlebar<R> {
  pub fn titlebar(&self) -> crate::Result<()> {
    Ok(())
  }
}
