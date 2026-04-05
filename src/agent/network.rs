use sysinfo::Networks;

pub fn get_local_ip() -> Option<String> {
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        if interface_name == "lo" || interface_name.starts_with("Loopback") {
            continue;
        }
        for ip in data.ip_networks() {
            let ip_str = ip.addr.to_string();
            if ip_str.starts_with("192.168.")
                || ip_str.starts_with("10.")
                || ip_str.starts_with("172.")
            {
                return Some(ip_str);
            }
        }
    }
    None
}
