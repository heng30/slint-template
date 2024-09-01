#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[tokio::main]
async fn main() {
    extern crate slint_template;
    slint_template::desktop_main().await;
}
