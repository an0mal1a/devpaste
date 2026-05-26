# DevPaste 🦀

A small pastebin-style REST API built with Rust, Axum, Tokio and SQLite.

This project was created mainly as a way to learn Rust backend development, async programming, API architecture, persistence and testing while building something practical and progressively improving it over time.

The goal is not to create a production-ready Pastebin clone, but to understand how modern Rust backend applications are structured and how async Rust works in real-world scenarios.

DevPaste did not start with all the current features. It started as a tiny Axum API, then got file persistence, then SQLite, then visibility/password rules, and now a small test suite to stop breaking things while changing the code.

---

## Features

Current implemented features:

- Create pastes
- List public, non-protected pastes
- Read a single paste by id
- Delete pastes
- Public and unlisted pastes
- Password-protected pastes
- JSON REST API
- SQLite persistence with `rusqlite`
- Automatic database/table initialization
- Password hashing with BLAKE3
- Constant-time password hash comparison
- Slug-based access for unlisted/protected pastes
- Function-level tests using isolated SQLite databases
- Async HTTP server with Tokio
- Axum routing and extractors

---

## Development progress

The project has been built progressively instead of being designed as a complete product from day one.

### First version

The first version focused on the basic REST API shape:

- Axum server setup
- Health check endpoint
- Create paste endpoint
- List pastes endpoint
- Read paste by id endpoint
- Delete paste endpoint
- Basic JSON request/response handling

At this stage the main goal was understanding Axum routing, handlers and extractors.

### File persistence

After the API was working, persistence was added using a local JSON file.

This helped keep the project simple while learning:

- Serialization and deserialization with Serde
- Reading and writing local files
- Keeping paste data between server restarts
- Separating API handlers from helper logic

### SQLite migration

The project then moved from JSON file storage to SQLite using `rusqlite`.

This introduced:

- A local SQLite database file
- Database initialization on startup
- A `pastebins` table
- SQL queries for creating, reading, listing and deleting pastes
- A cleaner path toward future database-backed features

### Visibility and protection

Later changes added more realistic paste behavior:

- Public pastes
- Unlisted pastes -> return a hash-based slug
- Password-protected pastes
- Passwords are stored as hashes, not plain text
- Protected pastes are automatically hidden from the public list
- Paste responses no longer expose the stored password
- Reading private pastes by numeric id returns the same error as a missing paste

> This started with passwords stored in plain text at 2:36AM. That was bad, but also part of the learning process. It now stores hashes instead.

This is still intentionally simple, but it makes the API closer to how a pastebin service would behave.

### Tests

The latest step was adding function-level tests.

This introduced:

- A `src/lib.rs` file so the project logic can be imported from integration tests
- Tests for creating public, unlisted and protected pastes
- Tests for reading public pastes, slug access and hidden/private behavior
- Tests for deleting public and protected pastes
- Temporary SQLite databases for tests through `DEVPASTE_DB_PATH`
- A first safety net before doing larger refactors

The tests are not HTTP/API tests yet. They currently test the Rust functions directly, which keeps them simpler while learning the basics.

---

## Tech stack

- Rust 2024 edition
- Axum
- Tokio
- Serde / Serde JSON
- BLAKE3
- constant_time_eq
- Rusqlite
- SQLite

---

## API Endpoints

All responses are JSON and use a simple `status` field with either `ok` or `ko`.

### Health check

```http
GET /
```

Example response:

```json
{
  "status": "ok",
  "message": "Server is running perfectly"
}
```

---

### List public pastes

```http
GET /pastes
```

Returns only pastes that are public and not password protected.

---

### Get a single paste

```http
GET /pastes/{id}
```

Protected and unlisted pastes are not readable from this endpoint by id.

Use the slug endpoint for unlisted/protected pastes.

---

### Get a paste by slug

```http
GET /p/{slug}
```

If the paste is password protected, send the password in the JSON body:

```json
{
  "password": "secret"
}
```

---

### Create a paste

```http
POST /pastes
```

Example public paste:

```json
{
  "title": "example",
  "content": "hello world"
}
```

Example unlisted paste:

```json
{
  "title": "unlisted note",
  "content": "only people with the id can read this (i know this can be insecure, its called IDOR)",
  "public": false
}
```

Example password-protected paste:

```json
{
  "title": "private note",
  "content": "protected content",
  "password": "secret"
}
```

Notes:

- `public` defaults to `true`.
- `password` defaults to an empty string.
- When a password is provided, the paste is marked as protected and forced to `public: false`.

---

### Delete a paste

```http
DELETE /pastes/{id}
```

If the paste is password protected, send the password in the JSON body:

```json
{
  "password": "secret"
}
```

---

## Running locally

### Clone repository

```bash
git clone https://github.com/an0mal1a/devpaste
cd devpaste
```

### Run project

```bash
cargo run
```

Server runs on:

```txt
http://localhost:8081
```

The SQLite database is stored in:

```txt
pastes.sql
```

---

## Project structure

```txt
src/
|-- lib.rs
|-- main.rs
|-- modules.rs
`-- utils.rs

tests/
|-- common/
|-- create_paste.rs
|-- delete_paste.rs
`-- read_paste.rs
```

### Main files

- `src/lib.rs`: exposes the modules so integration tests can import the project logic.
- `src/main.rs`: Axum router, endpoint handlers and server startup.
- `src/modules.rs`: Request/response/data structs.
- `src/utils.rs`: SQLite connection, database initialization and paste operations.
- `tests/`: function-level integration tests for paste creation, reading and deletion.

---

## Running tests

```bash
cargo test
```

Tests use isolated SQLite database files under `target/test-dbs` instead of touching the normal `pastes.sql` file.

---

## Current limitations

This project is intentionally simple and currently has several known limitations:

- No authentication
- No authorization for deleting pastes
- Password hashing is intentionally simple and does not use salts yet
- No async database driver
- No validation layer
- Minimal error handling
- Most errors are still plain strings instead of typed errors
- API errors still return JSON bodies instead of proper HTTP status mapping
- No pagination for listing pastes
- SQLite database file is local to the project
- Tests currently cover functions, not real HTTP requests

At this stage the project is focused on learning core Rust backend concepts rather than optimization or production readiness.

---

## Future ideas

Planned improvements and experiments:

- Password hashing 🟢
- Function-level tests 🟢
- Better typed error handling
- Expiring pastes
- Middleware and logging
- JWT authentication
- Rate limiting
- Shared application state
- Async database drivers
- PostgreSQL / MySQL support
- Docker support

## Future ideas (most probably not)

- Syntax highlighting (maybe)
- Web frontend (maybe)

---

## Why this project exists

The main purpose of DevPaste is learning.

This project is helping me understand:

- async/await in Rust
- Tokio runtime
- Axum routing system
- JSON extractors
- SQLite persistence
- SQL queries from Rust
- Integration tests
- Test isolation with temporary database files
- Ownership and borrowing in backend code
- Result-based error handling
- API architecture
- Serialization / deserialization
- Modular Rust project structure

---

## Notes

This is an experimental learning project and the codebase will evolve heavily over time as I continue exploring Rust backend development.

Contributions, suggestions and code reviews are always welcome 😄
