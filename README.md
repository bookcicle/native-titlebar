# Tauri Native Titlebar Plugin

A minimal Tauri plugin for macOS that creates a transparent title bar and adds a custom native button in the title bar region. It also disables dragging the window by the entire background, allowing only the title bar area (or a specific region) to handle window movement.

---

## Overview

This plugin demonstrates how to:
1. Make the macOS window’s title bar transparent.
2. Extend the content view to cover the title bar area.
3. Disable automatically dragging the window by the entire background (`[nsWindow setMovableByWindowBackground:NO]`).

> **Note**: This plugin is macOS-only. When building on other platforms, the code is conditionally excluded.

---

## Installation

1. **Add this plugin** as a dependency in your Cargo.toml:
   ```toml
   [dependencies]
   # ...
   native_titlebar_plugin = { path = "path/to/this/plugin" }
   
   # or 
   
   tauri-plugin-native-titlebar = { git = "https://github.com/bookcicle/native-titlebar.git" }
   ```

2. **Include the build script** in your `build.rs` (or ensure it’s part of your plugin’s build process). If you’re using the provided example `main()` function as-is, ensure it runs in your build pipeline:

   ```rust
   // build.rs or main() in the plugin crate
   #[cfg(target_os = "macos")]
   println!("cargo:rustc-link-lib=framework=AppKit");
   #[cfg(target_os = "macos")]
   println!("cargo:rustc-link-lib=framework=Foundation");

   #[cfg(target_os = "macos")]
   cc::Build::new()
       .file("src/native_titlebar.m")
       .flag("-fobjc-arc")
       .flag("-mmacosx-version-min=10.11")
       .compile("native-titlebar");
   ```

3. **Enable the plugin** in your Tauri application by referencing it in your code. Make sure the plugin’s commands are registered and built before your final Tauri build.

---

## Usage in Your Tauri App

1. **Register the command**: In the plugin code, we define:

   ```rust
   const COMMANDS: &[&str] = &["titlebar"];
   ```

   You should ensure your Tauri application includes this plugin’s command registration, typically via something like:

   ```rust
   use tauri::Manager;
   // ...
   tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![titlebar])  // Register the command
       // ...
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
   ```

2. **Call the command**: From your Tauri frontend (JavaScript/TypeScript), invoke the command:
   ```js
   window.__TAURI__.invoke("titlebar")
     .then(() => {
       console.log("Titlebar customized on macOS");
     })
     .catch((err) => {
       console.error("Failed to setup titlebar", err);
     });
   ```

   On macOS, this will:
    - Make the title bar transparent.
    - Extend your content view to the top.
    - Add a native button at the top-left corner.
    - Prevent the entire window background from being used to drag the window (only the title bar region can be used).

---

## How It Works

### `native_titlebar.m`

- **Title Bar Customization**
  ```objc
  [nsWindow setTitlebarAppearsTransparent:YES];
  nsWindow.styleMask |= NSWindowStyleMaskFullSizeContentView;
  ```
  This makes the title bar transparent and allows the content view to extend into the title bar region.

- **Disable Full-Window Dragging**
  ```objc
  [nsWindow setMovableByWindowBackground:NO];
  ```
  Stops the entire window background from being draggable. By default, macOS would treat the entire extended view as a draggable title bar.

- **Add a Button**
  ```objc
  NSView *contentView = [nsWindow contentView];
  NSRect contentFrame = [contentView frame];
  CGFloat buttonWidth = 80.0;
  CGFloat buttonHeight = 24.0;
  CGFloat margin = 8.0;
  // Logic to create and position NSButton (commented in the skeleton).
  ```
  The code positions a button near the top-left corner of the window. You can adjust dimensions and margins as needed.

- **Button Action**
  ```objc
  void customButtonPressed(id sender) {
      NSLog(@"Button Pressed!");
  }
  ```
  Currently, the action just logs a message to the console.

### `titlebar` Command

When you call `titlebar()`, Tauri will pass the native `NSWindow` pointer (`ns_window().unwrap()`) to our `setup_titlebar_buttons` function on macOS. That function applies all the title bar customizations and positions the button.

---

## Customizing the Draggable Region

Because `[nsWindow setMovableByWindowBackground:NO]` is set, only the standard “title bar” area is draggable. If you have a fully custom layout (for instance, an HTML/CSS interface in your Tauri app), you can specify draggable regions using CSS:

- **Make an element draggable:**
  ```css
  .titlebar-drag-region {
    -webkit-app-region: drag;
    -webkit-user-select: none;
  }
  ```
- **Exclude buttons or text from dragging:**
  ```css
  .no-drag {
    -webkit-app-region: no-drag;
  }
  ```

Ensure you position these elements at the top of your app where the title bar would normally be. Otherwise, the user may not be able to move the window at all.

---

## Further Modifications

- **Button Appearance**: You can style the native `NSButton` (or substitute an `NSView` and handle events yourself).
- **Window Controls**: If you want to reposition or hide the standard macOS traffic-light buttons, see Apple’s [NSWindow documentation](https://developer.apple.com/documentation/appkit/nswindow).
- **Platform Compatibility**: This code is macOS-specific. On other platforms, the code is conditionally excluded. If you need similar functionality on Windows or Linux, you’ll need platform-specific approaches.

---

## License

This code is provided as-is. Please adapt, extend, and use in accordance with your project’s needs and licensing requirements.