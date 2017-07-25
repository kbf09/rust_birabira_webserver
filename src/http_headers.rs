use std::collections::HashMap;

pub struct HttpHeaders {
    pub http_method : String,
    pub uri    : String,
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new(request_http_headers:&mut Vec<String>) -> HttpHeaders {

        // メソッド行だけ分離
        let mut raw_method_header = request_http_headers.remove(0);
        let mut a = raw_method_header.as_str();
        let mut method_header = a.split_whitespace();

        let method = method_header.next().unwrap();
        let uri = method_header.next().unwrap();
        let _ = method_header.next().unwrap();

        // メソッド行以外のヘッダ
        let mut headers = HashMap::new();
        for raw_header in request_http_headers {

            let mut header = raw_header.as_str();
            header = header.trim_right_matches("\n");
            header = header.trim_right_matches("\r");

            let tmp: Vec<&str> = header.splitn(2, ':').collect();
            if !tmp[0].is_empty() {
                headers.insert(tmp[0].to_string(), tmp[1].to_string());
            }
            
        }
        HttpHeaders {http_method: method.to_string(), uri: uri.to_string(), headers: headers}

    }
}


