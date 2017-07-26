
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::net::TcpListener;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use std::net::Shutdown;
use std::io::BufReader;
use std::vec::Vec;

mod http_headers;
mod webserver_config;



fn main() {

    println!("----------start----------");

    let config = webserver_config::WebserverConfig::new(r"D:\08_Desktop\config.json");
    println!("ip: {}, port: {}, root: {}", config.ip, config.port, config.document_root);

    let bind_addr = config.bind_addr();
    let listener = TcpListener::bind(bind_addr.as_str()).unwrap();

    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        let mut request_http_headers:Vec<String> = Vec::new();

        let mut reader = BufReader::new(s);
        
        loop {
            let mut buf: String = String::new();

            let len = match reader.read_line(&mut buf) {
                Ok(size) =>{size}
                Err(_) => {break}
            };

            if len == 0 || buf == "\r\n" {
                break;
            }
            request_http_headers.push(buf);
        }

        let http_headers = http_headers::HttpHeaders::new(&mut request_http_headers);
        let mut s = reader.get_mut();

        let mut f = File::open(r"D:\08_Desktop\projects\aaaaaaa\src\index.html").unwrap();
        let mut value = String::new();
        f.read_to_string(&mut value).unwrap();

        s.write(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n").unwrap();
        s.write(value.as_bytes()).unwrap();

        s.shutdown(Shutdown::Both).unwrap();

    }
}

