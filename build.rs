const COMMANDS: &[&str] = &["titlebar"];

fn main() {
  #[cfg(target_os = "macos")]
  println!("cargo:rustc-link-lib=framework=AppKit");
  #[cfg(target_os = "macos")]
  println!("cargo:rustc-link-lib=framework=Foundation");

  #[cfg(target_os = "macos")]
  cc::Build::new()
      .file("src/native_titlebar.m")
      // If you’re using ARC in that .m file, add:
      .flag("-fobjc-arc")
      .compile("native_titlebar");


  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
