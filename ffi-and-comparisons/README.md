# Systems Language Comparisons & FFI Guide 🌐

A guide comparing low-level language paradigms, FFI boundaries, and systems engineering trade-offs between **C**, **C++**, and **Rust**.

---

## 📁 Directory Structure

This directory is organized into modular language barriers:

- [**C Interop & FFI Barriers (`/c-and-ffi`)**](./c-and-ffi/README.md)  
  Focuses on raw C ABI interop (`extern "C"`), foreign function calling conventions, memory layout guarantees (`#[repr(C)]`), pointer passing, raw buffer ownership transfers, and `bindgen`.

- [**C++ to Rust Systems Paradigms (`/cpp-to-rust`)**](./cpp-to-rust/README.md)  
  Focuses on mapping high-level C++ abstractions (`std::unique_ptr`, `std::shared_ptr`, `std::vector`, templates, RAII constructors/destructors, virtual inheritance) directly to idiomatic Rust primitives (`Box`, `Arc`, `Vec`, generics, `Drop`, `dyn Trait`).

> 💡 *Future Language Extensions*: Additional subdirectories (e.g., `/python-pyo3`, `/go-cgo-ffi`) can be added under this directory while preserving clear isolation barriers.
