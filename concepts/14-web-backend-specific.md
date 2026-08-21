# Topic 14: Web / Backend-Specific (Actix-Web / Axum / General)

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What is the extractor pattern in a web framework like Actix-Web or Axum, and what trait powers it (`FromRequest` / `FromRequestParts`)?**
   * *Key aspects to address:* Strongly-typed HTTP request extraction (headers, path params, JSON payloads), trait implementations parsing payload buffers into type-safe parameters before handler invocation.

2. **How would you structure error handling across HTTP handlers in a Rust web service?**
   * *Key aspects to address:* Custom application error type implementing framework-specific HTTP response conversion traits (`ResponseError` in Actix, `IntoResponse` in Axum), mapping internal domain errors to HTTP status codes & JSON payload structures.

3. **What's the difference between SQLx and an ORM-style crate (like Diesel or SeaORM), in terms of compile-time guarantees?**
   * *Key aspects to address:* Compile-time SQL query validation against a live database or cached JSON schema metadata (`sqlx::query!`) vs Rust macro DSL mapping relational schemas to Rust structs.

4. **How would you design a streaming file/image-serving endpoint efficiently?**
   * *Key aspects to address:* Non-blocking chunked streaming (`tokio::fs::File`, `tokio_util::io::ReaderStream`), constant memory consumption regardless of payload size, handling client backpressure, zero-copy buffer passing (`bytes::Bytes`).
