extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Read;



#[derive(Serialize, Deserialize)]
pub struct WebserverConfig {
    pub ip: String,
    pub port: String,
    pub document_root: String,
}

impl WebserverConfig {
    pub fn new(path: &str) -> WebserverConfig {

        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(_) => panic!("config file not found!"),
        };
        let mut config = String::new();
        f.read_to_string(&mut config).unwrap();

        let a : WebserverConfig = serde_json::from_str(config.as_str()).unwrap();
        a
    }

    pub fn bind_addr(&self) -> String {
        let ip = self.ip.as_str();
        let port = self.port.as_str();

        format!("{}:{}", ip, port)
    }
}