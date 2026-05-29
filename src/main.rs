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

    if args.len() >= 3 {
        let index_arg = &args[2];
        match index_arg.parse::<usize>() {
            Ok(idx) => {
                if idx > 0 && idx <= parsed_request.len() {
                    let target_req = &parsed_request[idx - 1];
                    send_request(target_req);
                } else {
                    println!(
                        "Error: Request index {} is out of bounds. This file has {} requests.",
                        idx,
                        parsed_request.len(),
                    );
                }
            }
            Err(_) => {
                println!(
                    "Error: '{}' is not a valid number. Please provide a request index like '1' or '2'.",
                    index_arg
                );
            }
        }
    } else {
        for req in &parsed_request {
            send_request(req);
        }
    }

    Ok(())
}
