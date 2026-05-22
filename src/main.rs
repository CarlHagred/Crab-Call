use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug)]
struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[derive(Debug, PartialEq)]
enum HttpToken {
    Variable(String, String),
    Separator,
    RequestLine(String, String),
    Header(String, String),
    Body(String),
    Comment(String),
}

fn tokenize(content: &str) -> Vec<HttpToken> {
    let mut tokens = Vec::new();
    let mut in_body = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("###") {
            in_body = false;
            tokens.push(HttpToken::Separator);
        } else if in_body {
            if trimmed.is_empty() {
                continue;
            }
            tokens.push(HttpToken::Body(trimmed.to_string()))
        } else if trimmed.is_empty() {
            in_body = true;
            continue;
        } else if trimmed.starts_with("#") {
            tokens.push(HttpToken::Comment(trimmed.to_string()));
        } else if let Some((name, value)) =
            trimmed.strip_prefix('@').and_then(|s| s.split_once('='))
        {
            tokens.push(HttpToken::Variable(name.to_string(), value.to_string()))
        } else if trimmed.starts_with("GET ")
            || trimmed.starts_with("POST ")
            || trimmed.starts_with("PUT ")
            || trimmed.starts_with("DELETE ")
        {
            match trimmed.split_once(' ') {
                Some((method, url)) => {
                    tokens.push(HttpToken::RequestLine(method.to_string(), url.to_string()))
                }
                None => println!("The Request lines formating is worng"),
            };
        } else if let Some((key, value)) = trimmed.split_once(':') {
            tokens.push(HttpToken::Header(key.to_string(), value.to_string()))
        }
    }
    tokens
}

fn inject_variable(text: &str, variables: &HashMap<String, String>) -> String {
    let mut result = text.to_string();

    for (key, value) in variables {
        let target = format!("{{{{{}}}}}", key);

        result = result.replace(&target, value);
    }

    result
}

fn parse_requests(tokens: Vec<HttpToken>) -> Vec<HttpRequest> {
    let mut current_method: Option<HttpMethod> = None;
    let mut variable: HashMap<String, String> = HashMap::new();
    let mut current_url: Option<String> = None;
    let mut current_headers: HashMap<String, String> = HashMap::new();
    let mut current_body_lines: Vec<String> = Vec::new();
    let mut request = Vec::new();

    for token in tokens {
        match token {
            HttpToken::Variable(name, value) => {
                variable.insert(name, value);
            }
            HttpToken::RequestLine(method_str, url_str) => {
                current_method = match method_str.as_str() {
                    "POST" => Some(HttpMethod::Post),
                    "GET" => Some(HttpMethod::Get),
                    "PUT" => Some(HttpMethod::Put),
                    "DELETE" => Some(HttpMethod::Delete),
                    _ => None,
                };
                let injected_url = inject_variable(&url_str, &variable);
                current_url = Some(injected_url);
            }
            HttpToken::Header(key, value) => {
                current_headers.insert(key, value);
            }
            HttpToken::Body(line) => {
                current_body_lines.push(line);
            }
            HttpToken::Separator => {
                let final_body = current_body_lines.join("\n");
                let body_option = if final_body.is_empty() {
                    None
                } else {
                    Some(final_body)
                };
                let ready_headers = current_headers;
                current_headers = HashMap::new();
                if let (Some(method), Some(url)) = (current_method.take(), current_url.take()) {
                    let req = HttpRequest {
                        method,
                        url,
                        headers: ready_headers,
                        body: body_option,
                    };
                    request.push(req);
                }
                current_body_lines.clear();
            }
            _ => {}
        }
    }
    let final_body = current_body_lines.join("\n");
    let body_option = if final_body.is_empty() {
        None
    } else {
        Some(final_body)
    };

    if let (Some(method), Some(url)) = (current_method, current_url) {
        let req = HttpRequest {
            method,
            url,
            headers: current_headers,
            body: body_option,
        };
        request.push(req);
    }

    request
}

fn send_request(req: &HttpRequest) {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: crabcall <file.http>");
        return Ok(());
    }
    let raw_text = fs::read_to_string(&args[1])?;

    let my_tokens = tokenize(raw_text.trim());
    println!("{:#?}", my_tokens);

    let parsed_request = parse_requests(my_tokens);
    println!("Parsed request:\n{:?}", parsed_request);

    for req in &parsed_request {
        send_request(req);
    }

    Ok(())
}
