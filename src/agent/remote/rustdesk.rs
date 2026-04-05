use std::fs;

pub fn get_id() -> Option<String> {
    get_via_cli().or_else(get_via_toml)
}

fn get_via_cli() -> Option<String> {
    let exe_paths = vec![
        "C:\\Program Files\\RustDesk\\rustdesk.exe".to_string(),
        "C:\\Program Files (x86)\\RustDesk\\rustdesk.exe".to_string(),
        "rustdesk".to_string(),
    ];

    for exe in &exe_paths {
        let mut cmd = std::process::Command::new(exe);
        cmd.arg("--get-id");

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        if let Ok(output) = cmd.output() {
            let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !id.is_empty() {
                return Some(id);
            }
        }
    }
    None
}

fn get_via_toml() -> Option<String> {
    let paths = vec![
        format!(
            "C:\\Users\\{}\\AppData\\Roaming\\RustDesk\\config\\RustDesk.toml",
            std::env::var("USERNAME").unwrap_or_default()
        ),
        format!(
            "{}/.config/rustdesk/RustDesk.toml",
            std::env::var("HOME").unwrap_or_default()
        ),
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(parsed) = content.parse::<toml::Table>() {
                let id = parsed
                    .get("id")
                    .and_then(|v| v.as_str())
                    .or_else(|| parsed.get("enc_id").and_then(|v| v.as_str()));
                if let Some(id_val) = id {
                    return Some(id_val.to_string());
                }
            }
        }
    }
    None
}
