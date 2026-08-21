# Topic 01: Ownership, Borrowing & Lifetimes

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What is ownership, and why did Rust choose this model over garbage collection?**
   * *Key aspects to address:* Single owner rule, compile-time scope-based deallocation (`Drop`), memory safety without runtime GC pause overhead, predictable resource management.

2. **What's the difference between a move and a copy? Which types implement `Copy`?**
   * *Key aspects to address:* Bitwise copy (`memcpy`) vs transfer of ownership, `Copy` trait requirements (types with no custom `Drop`, scalar primitives, fixed-size arrays/tuples of `Copy` types).

3. **Explain the borrow checker's rules. Why can't you have a mutable reference and an immutable reference at the same time?**
   * *Key aspects to address:* Aliasing XOR Mutability theorem (data race freedom at compile time), reader-writer exclusivity, non-lexical lifetimes (NLL).

4. **What are lifetimes, and why does the compiler need them?**
   * *Key aspects to address:* Generic lifetime parameters (`'a`), ensuring references remain valid for as long as the underlying data exists, preventing use-after-free at compile time.

5. **What is lifetime elision? Give an example where it kicks in vs where you must annotate manually.**
   * *Key aspects to address:* The 3 elision rules for function parameters/returns, single input lifetime propagation, `&self`/`&mut self` rule, multi-reference input functions returning a reference requiring explicit lifetime bounds.

6. **What's the difference between `'static` lifetime and a `'static` trait bound on a generic (`T: 'static`)?**
   * *Key aspects to address:* Reference lasting for the entire execution of the binary vs type holding no non-`'static` references (owned types like `String` satisfy `T: 'static`).

7. **What is a dangling reference, and how does Rust prevent it at compile time?**
   * *Key aspects to address:* Stack frame destruction, returning reference to local variable, borrow checker lifetime inference catching reference outliving target scope.

8. **Explain `Cow<'a, T>` (Clone-on-Write) and when you'd use it.**
   * *Key aspects to address:* `Borrowed(&'a T)` vs `Owned(<T as ToOwned>::Owned)`, avoiding allocations when read-only modifications are conditional (e.g., URL unescaping, string formatting).
