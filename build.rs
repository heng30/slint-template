use cmd_lib::run_fun;
use std::fs::File;
use std::io::Write;

fn main() {
    // #[cfg(target_os = "windows")] would not work in cross-compiling on a non-windows OS
    // otherwise, using `CARGO_CFG_TARGET_OS`
    // if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {}

    #[cfg(target_os = "windows")]
    set_windows_info();

    let _ = write_app_version();
}

#[allow(unused)]
fn build_log(msg: &str) {
    let mut file = File::create("build.log").unwrap();
    _ = file.write(msg.as_bytes());
}

fn write_app_version() -> Result<(), Box<dyn std::error::Error>> {
    let tags = run_fun!(git describe --tags --abbrev=0)?
        .split(char::is_whitespace)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let output = if let Some(version) = tags.last() {
        format!(r#"pub static VERSION: &str = "{}";"#, version)
    } else {
        format!(r#"pub static VERSION: &str = "{}";"#, "0.0.1")
    };

    let _ = std::fs::write("src/version.rs", output);

    Ok(())
}

#[cfg(target_os = "windows")]
fn set_windows_info() {
    embed_resource::compile("./windows/icon.rc", embed_resource::NONE);
}
