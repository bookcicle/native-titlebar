use std::ffi::c_void;
use tauri::{command, Runtime, Window};


#[command]
pub fn titlebar<R: Runtime>(window: Window<R>) {
    #[cfg(target_os = "macos")]
    unsafe {
        setup_titlebar_buttons(window.ns_window().unwrap());
    }
}

#[cfg(target_os = "macos")]
extern "C" {
    fn setup_titlebar_buttons(ns_window: *mut c_void);
}