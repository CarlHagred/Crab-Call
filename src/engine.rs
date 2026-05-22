use crate::models::{HttpMethod, HttpRequest};

pub fn send_request(req: &HttpRequest) {
    let client = reqwest::blocking::Client::new();

    let mut builder = match req.method {
        HttpMethod::Get => client.get(&req.url),
        HttpMethod::Post => client.post(&req.url),
        HttpMethod::Put => client.put(&req.url),
        HttpMethod::Delete => client.delete(&req.url),
    };
    for (key, value) in &req.headers {
        builder = builder.header(key, value);
    }

    if let Some(body_content) = &req.body {
        builder = builder.body(body_content.clone());
    }

    let response = builder.send().expect("Failed to send HTTP request");

    println!("--- RESPONSE ---");
    println!("Status: {}", response.status());
    println!("Body:\n{}", response.text().unwrap_or_default());
    println!("----------------\n");
}
