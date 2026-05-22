# Crab Call

A Rust HTTP client that parses and executes `.http` files.

## Overview

Crab Call reads a `.http` file format tokenizes it, resolves variables,
and executes the defined HTTP requests.

## Features

- **HTTP methods** -- GET, POST, PUT, DELETE
- **Variables** -- define with `@name=value`, interpolate with `{{name}}`
- **Headers** -- standard `Key: Value` format
- **Request bodies** -- separated from headers by a blank line
- **Request separation** -- split multiple requests with `###`
- **Comments** -- lines starting with `#`

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)

### Build and run

```sh
cargo build
cargo run
```

## Usage

crab_call parses the following .http syntax:

```http
# Define variables
@base_url=https://api.example.com
@token=my-secret-token

### GET request with variable interpolation
GET {{base_url}}/users
Authorization: Bearer {{token}}
Content-Type: application/json

### POST request with a body
POST {{base_url}}/users
Content-Type: application/json

{"name": "Alice", "email": "alice@example.com"}
```

Each request is separated by `###`.
Variables defined with `@` are available for interpolation in all subsequent requests.

## Project Structure

```
crab_call/
├── Cargo.toml     # Package manifest and dependencies
├── LICENSE        # MIT license
└── src/
    └── main.rs    # Tokenizer, parser, and HTTP executor
```

## License

MIT
