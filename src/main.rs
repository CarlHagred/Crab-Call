use std::env;
use std::fs;

mod models;

mod lexer;
use lexer::tokenize;

mod parser;
use parser::parse_requests;

mod engine;
use engine::send_request;

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
