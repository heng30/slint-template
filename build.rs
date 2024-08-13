use cmd_lib::run_fun;

fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();

    #[cfg(target_os = "windows")]
    {
        set_win_info();
    }

    let _ = write_app_version();
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
fn set_win_info() {
    embed_resource::compile("./win/icon.rc", embed_resource::NONE);
}
