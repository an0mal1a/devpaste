# DevPaste 🦀

A small pastebin-style REST API built with Rust, Axum and Tokio.

This project was created mainly as a way to learn Rust backend development, async programming and API architecture while building something practical and progressively improving it over time.

The goal is not to create a production-ready Pastebin clone, but to understand how modern Rust backend applications are structured and how async Rust works in real-world scenarios.

---

## Features

Current implemented features:

- Create pastes
- List all pastes
- Read a single paste
- Delete pastes
- JSON REST API
- File-based persistence using JSON
- Async HTTP server with Tokio
- Axum routing & extractors

---

## Tech stack

- Rust
- Axum
- Tokio
- Serde / Serde JSON
- Chrono

---

## API Endpoints

### Health check

```http
GET /
```
---


### List all pastes

```http
GET /pastes
```

---

### Get a single paste

```http
GET /pastes/{id}
```

---

### Create a paste

```http
POST /pastes
```

Example body:

```json
{
    "title": "example",
    "content": "hello world"
}
```

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
http://localhost:8080
```

---

## Project structure

```txt
src/
├── main.rs
├── modules.rs
└── utils.rs
```

---

## Current limitations

This project is intentionally simple and currently has several known limitations:

* File-based storage
* No authentication
* No database
* No async file handling
* Possible race conditions on concurrent writes
* No validation layer
* Minimal error handling

At this stage the project is focused on learning core Rust backend concepts rather than optimization or production readiness.

---

## Future ideas

Planned improvements and experiments:

* SQLite support 🟢
* Private/unlisted pastes 🟢
* Password-protected pastes 🟢
* Crypted password-protected pastes (like if the user has a pro plan can crypt)

* PostgreSQL / MySQL support
* Expiring pastes
* Better error handling
* Middleware & logging
* JWT authentication
* Rate limiting
* Shared application state
* Async database drivers

## Future ideas (most probabbly not)
* Syntax highlighting
* Docker support
* Web frontend

---

## Why this project exists

The main purpose of DevPaste is learning.

This project is helping me understand:

* async/await in Rust
* Tokio runtime
* Axum routing system
* JSON extractors
* Shared state management
* Ownership & borrowing in backend code
* Result-based error handling
* API architecture
* Serialization / deserialization
* File handling
* Modular Rust project structure

---

## Notes

This is an experimental learning project and the codebase will evolve heavily over time as I continue exploring Rust backend development.

Contributions, suggestions and code reviews are always welcome 😄

---
