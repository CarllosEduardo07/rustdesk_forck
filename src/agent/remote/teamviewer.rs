use std::fs;

pub fn get_id() -> Option<String> {
    get_from_os()
}

#[cfg(target_os = "windows")]
fn get_from_os() -> Option<String> {
    let queries = [
        "HKLM:\\SOFTWARE\\WOW6432Node\\TeamViewer",
        "HKLM:\\SOFTWARE\\TeamViewer",
    ];

    for path in &queries {
        let command = format!(
            "(Get-ItemProperty -Path '{}' -Name ClientID -ErrorAction SilentlyContinue).ClientID",
            path
        );

        let mut cmd = std::process::Command::new("powershell");
        cmd.args(["-NoProfile", "-NonInteractive", "-Command", &command]);

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

#[cfg(not(target_os = "windows"))]
fn get_from_os() -> Option<String> {
    let paths = vec![
        "/etc/teamviewer/global.conf".to_string(),
        format!(
            "{}/.config/teamviewer/global.conf",
            std::env::var("HOME").unwrap_or_default()
        ),
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if line.contains("ClientID") {
                    if let Some(id) = line.split('=').nth(1) {
                        let id = id.trim().to_string();
                        if !id.is_empty() {
                            return Some(id);
                        }
                    }
                }
            }
        }
    }
    None
}
