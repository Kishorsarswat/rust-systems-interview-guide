# Topic 08: Macros

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What's the difference between declarative macros (`macro_rules!`) and procedural macros?**
   * *Key aspects to address:* Pattern-matching AST fragment syntax (`expr`, `ident`, `ty`, `path`) vs custom Rust functions operating directly on `proc_macro::TokenStream`.

2. **How do you create a simple `macro_rules!` macro? Walk through matching patterns and repetition.**
   * *Key aspects to address:* Matcher syntax, designators (`$e:expr`), repetition operators (`*`, `+`, `?`), expansion templates, macro scope exporting `#[macro_export]`.

3. **What are the three kinds of procedural macros (`derive`, `attribute`, `function-like`)?**
   * *Key aspects to address:* Custom derive attributes (`#[derive(MyTrait)]`), custom attribute decorators (`#[tokio::main]`), function-like macro calls (`sqlx::query!`).

4. **What is macro hygiene, and why does it matter?**
   * *Key aspects to address:* Variable shadowing prevention, identifier isolation, preventing macro expansion from inadvertently reading or modifying caller scope variables (`proc_macro2::Span`).

5. **When would you reach for a macro instead of a generic function?**
   * *Key aspects to address:* Variadic arguments (`println!`, `vec!`), compile-time code generation / DSL creation, syntax manipulation, conditionally generating code structures based on attributes.

6. **What are the downsides of heavy macro use (compile time, debuggability, error messages)?**
   * *Key aspects to address:* Compilation speed degradation (`syn`/`quote` parsing overhead), cryptic compiler error tracebacks, IDE code intelligence / autocomplete friction (`rust-analyzer` expansion cost).
