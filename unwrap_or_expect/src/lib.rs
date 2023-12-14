pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            return server.clone().expect("called `Result::unwrap()` on an `Err` value");
        }
        Security::High => {
            if let Err(ref err) = &server {
                panic!("ERROR: program stops: {}", err);
            } else {
                server.as_ref().expect("Panicking without custom message");
            }
        }
        Security::Medium => {
            if server.is_err() {
                return "WARNING: check the server".to_string();
            }
        }
        Security::Low => {
            if let Err(server_url) = &server {
                return format!("Not found: {}", server_url);
            }
        }
        Security::BlockServer => {
            if let Ok(ok_msg) = &server {
                panic!("{}", ok_msg);
            }
        }
    }
    server.unwrap_or_else(|err| err)
}