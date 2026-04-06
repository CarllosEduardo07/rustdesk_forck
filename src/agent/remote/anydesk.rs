use std::fs;

pub fn get_id() -> Option<String> {
    let paths = vec![
        "C:\\ProgramData\\AnyDesk\\system.conf".to_string(),
        "/etc/anydesk/system.conf".to_string(),
        "/Library/Application Support/AnyDesk/system.conf".to_string(),
        format!(
            "C:\\Users\\{}\\AppData\\Roaming\\AnyDesk\\system.conf",
            std::env::var("USERNAME").unwrap_or_default()
        ),
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if line.starts_with("ad.anynet.id=") {
                    let id = line.trim_start_matches("ad.anynet.id=").trim().to_string();
                    if !id.is_empty() {
                        return Some(id);
                    }
                }
            }
        }
    }
    None
}
