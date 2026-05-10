use std::collections::HashMap;

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

fn main() {
    let mut my_headers = HashMap::new();
    my_headers.insert("Content-Type".to_string(), "application/json".to_string());
    let my_http_request = HttpRequest {
        method: HttpMethod::Post,
        url: "https://webhook.site/c07ba92e-27c2-455a-b2c5-dc762d9eb1a8".to_string(),
        headers: my_headers,
        body: Some("Hello World".to_string()),
    };
    println!("{:#?}", my_http_request);

    let raw_text = "
    @base_url=http://127.0.0.1:8000
    ###
    POST {{base_url}}/todoitems
    Content-Type: application/json
    Authorization: Bearer my-token

    {
        \"name\": \"walk dog\",
        \"isComplete\": false
    }
    ";

    let my_tokens = tokenize(raw_text.trim());
    println!("{:#?}", my_tokens);
}
