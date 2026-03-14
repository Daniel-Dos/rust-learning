# CLAUDE.md

This file provides guidance to Claude (and other AI assistants) when working with this repository.

---

## Project Overview

A Rust learning project implementing a full **User CRUD** backed by **SQLite**. The application is asynchronous and follows a layered architecture: Presentation → Service → Repository → Database.

---

## Architecture

```
app/src/
├── main.rs                     # Entry point, orchestrates the CRUD flow
├── lib.rs                      # Re-exports all public modules
├── models/
│   └── user.rs                 # User struct with getters (id, username, email, age)
├── repository/
│   ├── db_sqlite.rs            # SQLx async SQLite repository (primary)
│   └── db_memory.rs            # In-memory HashMap repository (for reference/testing)
├── service/
│   └── user_service.rs         # Business logic layer wrapping the repository
└── utils/
    └── utils.rs                # Random data generators (username, email, age)
```

---

## Common Commands

All commands must be run from the `app/` directory:

```bash
# Run the application
cargo run

# Run all tests
cargo test --verbose

# Build (debug)
cargo build

# Build (release / Docker)
cargo build --release
```

### Docker

```bash
# Build image (run from app/ directory)
docker build -t rust-learning .

# Run container
docker run rust-learning
```

> **Note:** The SQLite database file `user-rust.db` must exist before running.  
> The Dockerfile copies it automatically during the image build.

---

## Database

- **Engine:** SQLite via [SQLx](https://github.com/launchbay/sqlx)
- **File:** `app/user-rust.db`
- **Schema:**

```sql
CREATE TABLE users (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT    NOT NULL,
    email    TEXT    NOT NULL,
    age      INTEGER NOT NULL
);
```

SQLx migrations are enabled (`migrate` feature) but not yet used — migrations folder is not present. If you add one, place it at `app/migrations/`.

---

## Key Dependencies

| Crate               | Version       | Purpose                          |
|---------------------|---------------|----------------------------------|
| `tokio`             | 1.49          | Async runtime (`rt-multi-thread`)|
| `sqlx`              | 0.8           | Async SQLite driver + macros     |
| `anyhow`            | 1.0           | Ergonomic error handling         |
| `tracing`           | 0.1           | Structured logging               |
| `tracing-subscriber`| 0.3           | Log formatting to stdout         |
| `rand`              | 0.10.0-rc.8   | Random data generation           |

> `rand` is a **release candidate**. Upgrade to stable once available.

---

## Testing

Integration tests live in `app/tests/repository_tests.rs` and use `#[sqlx::test]`, which automatically provisions an in-memory SQLite database per test — no external setup needed.

```bash
cargo test --verbose
```

Covered scenarios:
- Save a user and retrieve all users
- Delete a user by ID
- Update a user's email by ID

When adding new repository methods, add a corresponding `#[sqlx::test]` in `tests/repository_tests.rs`.

---

## CI/CD

GitHub Actions workflow (`.github/workflows/rust.yml`) runs on every push/PR to `master`:

1. Checks out the repository
2. Runs `cargo build --verbose` from `./app`
3. Runs `cargo test --verbose` from `./app`

---

## Code Conventions

- **Error handling:** Use `anyhow::Result` at the application layer; propagate `sqlx::Error` from repository/service layers.
- **Logging:** Use `tracing` macros (`info!`, `error!`) — never `println!` in production paths.
- **Async:** All I/O-bound operations must be `async`. Use `.await?` for error propagation.
- **Getters:** The `User` model exposes data only through getter methods — do not add `pub` fields directly.
- **New repository methods:** Must be added to `UserDBSqlite` (`db_sqlite.rs`) and exposed via `UserService` (`user_service.rs`).

---

## What NOT to Do

- Do not modify `db_memory.rs` for production features — it exists only as a reference/learning artifact.
- Do not commit `user-rust.db` with real data.
- Do not add `println!` — use `tracing` instead.
- Do not run `cargo` commands from the repository root — always `cd app` first.
