mod network;
mod remote;

use sysinfo::System;

pub fn collect_and_send() {
    let hostname = System::host_name().unwrap_or_else(|| "desconhecido".to_string());
    let local_ip = network::get_local_ip().unwrap_or_else(|| "não encontrado".to_string());
    let rustdesk_id = remote::rustdesk::get_id().unwrap_or_else(|| "não instalado".to_string());
    let anydesk_id = remote::anydesk::get_id().unwrap_or_else(|| "não instalado".to_string());
    let teamviewer_id = remote::teamviewer::get_id().unwrap_or_else(|| "não instalado".to_string());

    let payload = serde_json::json!({
        "hostname": hostname,
        "local_ip": local_ip,
        "rustdesk_id": rustdesk_id,
        "anydesk_id": anydesk_id,
        "teamviewer_id": teamviewer_id,
    });

    // Dispara em thread separada para não travar o RustDesk
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        match client
            .post("http://192.168.0.4:3000/api/agent")
            .json(&payload)
            .send()
        {
            Ok(res) => log::info!("Agent: enviado! Status: {}", res.status()),
            Err(e) => log::error!("Agent: erro ao enviar: {}", e),
        }
    });
}