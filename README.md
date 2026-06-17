# Crab Call

A Rust HTTP client that parses and executes `.http` files.

## Features

- **HTTP methods** -- GET, POST, PUT, DELETE
- **Variables** -- define with `@name=value`, interpolate with `{{name}}`

## Build and run

```sh
cargo build
cargo run -- test.http
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

## License

MIT
