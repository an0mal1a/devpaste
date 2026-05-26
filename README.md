# DevPaste

A small pastebin-style REST API built with Rust, Axum, Tokio and SQLite.

This project was created mainly as a way to learn Rust backend development, async programming, API architecture and persistence while building something practical and progressively improving it step by step.

The goal is not to create a production-ready Pastebin clone, but to understand how modern Rust backend applications are structured and how async Rust works in real-world scenarios.

DevPaste did not start with all the current features. The project is evolving gradually: first a basic Axum API, then simple persistence, then SQLite, and now extra paste visibility and protection features.

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

The latest changes add more realistic paste behavior:

- Public pastes
- Unlisted pastes -> 🟢 Return an hash url of content + title
- Password-protected pastes
- Protected pastes are automatically hidden from the public list
- Paste responses no longer expose the stored password

> (hello, this is fixed btw) By the time im writing this, the password is stored in plain text, its 2:36AM and i dont really care, good night.

This is still intentionally simple, but it makes the API closer to how a pastebin service would behave.

---

## Tech stack

- Rust 2024 edition
- Axum
- Tokio
- Serde / Serde JSON
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
|-- main.rs
|-- modules.rs
`-- utils.rs
```

### Main files

- `src/main.rs`: Axum router, endpoint handlers and server startup.
- `src/modules.rs`: Request/response/data structs.
- `src/utils.rs`: SQLite connection, database initialization and paste operations.

---

## Current limitations

This project is intentionally simple and currently has several known limitations:

- No authentication
- No authorization for deleting pastes
- Passwords are stored in plain text
- No password hashing or encryption
- No async database driver
- No validation layer
- Minimal error handling
- No pagination for listing pastes
- SQLite database file is local to the project

At this stage the project is focused on learning core Rust backend concepts rather than optimization or production readiness.

---

## Future ideas

Planned improvements and experiments:

- Password hashing 🟢
- Better error handling 🟢
- Expiring pastes
- Middleware and logging
- JWT authentication
- Rate limiting
- Shared application state
- Async database drivers
- PostgreSQL / MySQL support
- Docker support
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
- Ownership and borrowing in backend code
- Result-based error handling
- API architecture
- Serialization / deserialization
- Modular Rust project structure

---

## Notes

This is an experimental learning project and the codebase will evolve heavily over time as I continue exploring Rust backend development.

Contributions, suggestions and code reviews are always welcome.
