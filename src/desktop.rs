#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() {
    extern crate slint_template;
    slint_template::desktop_main().await;
}
