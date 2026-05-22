use crate::lexer::HttpToken;
use crate::models::{HttpMethod, HttpRequest};
use std::collections::HashMap;

fn inject_variable(text: &str, variables: &HashMap<String, String>) -> String {
    let mut result = text.to_string();

    for (key, value) in variables {
        let target = format!("{{{{{}}}}}", key);

        result = result.replace(&target, value);
    }

    result
}

pub fn parse_requests(tokens: Vec<HttpToken>) -> Vec<HttpRequest> {
    let mut current_method: Option<HttpMethod> = None;
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut current_url: Option<String> = None;
    let mut current_headers: HashMap<String, String> = HashMap::new();
    let mut current_body_lines: Vec<String> = Vec::new();
    let mut request = Vec::new();

    for token in tokens {
        match token {
            HttpToken::Variable(name, value) => {
                variables.insert(name, value);
            }
            HttpToken::RequestLine(method_str, url_str) => {
                current_method = match method_str.as_str() {
                    "POST" => Some(HttpMethod::Post),
                    "GET" => Some(HttpMethod::Get),
                    "PUT" => Some(HttpMethod::Put),
                    "DELETE" => Some(HttpMethod::Delete),
                    _ => None,
                };
                let injected_url = inject_variable(&url_str, &variables);
                current_url = Some(injected_url);
            }
            HttpToken::Header(key, value) => {
                current_headers.insert(key, inject_variable(&value, &variables));
            }
            HttpToken::Body(line) => {
                current_body_lines.push(inject_variable(&line, &variables));
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
