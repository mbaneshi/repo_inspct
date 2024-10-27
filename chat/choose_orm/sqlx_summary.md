**Timestamp:** 2024-10-27 17:05 UTC  
**Summary:** Summary of SQLx, an asynchronous Rust SQL toolkit with features and installation instructions.  
**Response Length:** 15 lines, 885 characters  

---

### SQLx Overview

**SQLx** is an asynchronous SQL toolkit for Rust, designed to support compile-time checked queries without needing a DSL. It supports multiple databases, including PostgreSQL, MySQL, MariaDB, and SQLite. Key features include:

- **Asynchronous Support**: Built using async/await for high concurrency.
- **Compile-time Checked Queries**: Ensures SQL queries are valid against a database.
- **Pure Rust Implementation**: Most drivers are written in safe Rust with minimal unsafe code.
- **Cross-platform**: Compiles wherever Rust is supported.
- **Built-in Connection Pooling** and **Row Streaming**: Efficiently manage database connections and data retrieval.

#### Installation

Add SQLx to your `Cargo.toml`, specifying runtime and TLS features. Example:
```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-native-tls"] }
```

#### Usage

To use SQLx, establish a connection pool and perform queries with compile-time verification using macros like `sqlx::query!`.

For comprehensive documentation, visit [SQLx Docs](https://docs.rs/sqlx).

```bash
nvim sqlx_summary.md
```
