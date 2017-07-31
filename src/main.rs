
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod bira;
use bira::webserver;

fn main() {

    println!("----------start----------");
    let webserver = webserver::Webserver::new(r"D:\08_Desktop\config.json");
    webserver.start();
    


}

