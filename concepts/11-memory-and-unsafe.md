# Topic 11: Memory & Unsafe

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What does `unsafe` actually allow you to do that safe Rust doesn't?**
   * *Key aspects to address:* The 5 unsafe superpowers: dereferencing raw pointers, calling unsafe functions/FFI, implementing unsafe traits (`Send`/`Sync`), mutating static mutable state, accessing fields of `union`s. (Note: unsafe does NOT disable borrow checker checks on references!).

2. **What is `MaybeUninit<T>`, and why would you use it over a normally-initialized value?**
   * *Key aspects to address:* Uninitialized memory handling without triggering undefined behavior (UB), incremental buffer initialization, avoiding zero-filling performance overhead (`assume_init()`).

3. **What are `union`s in Rust, and how do they differ from `enum`s?**
   * *Key aspects to address:* C-style untagged union layout (fields sharing the exact same memory space) vs type-safe tagged enum layout (discriminant + payload). Read operations on `union` fields requiring `unsafe`.

4. **What is a self-referential struct, and why is it hard to express in safe Rust?**
   * *Key aspects to address:* Struct containing a field that holds a reference to another field within the same struct, reference invalidation on struct move/reallocation, safe solutions (`Pin`, indices, rental crates).

5. **What are raw pointers (`*const T`, `*mut T`), and how do they differ from references (`&T`, `&mut T`)?**
   * *Key aspects to address:* Nullability, aliasing rules, variance, non-guaranteed validity/alignment, lack of lifetime tracking, explicit dereferencing in `unsafe` blocks.

6. **What invariants must you uphold when writing unsafe code to avoid undefined behavior (UB)?**
   * *Key aspects to address:* Pointer alignment, non-null validity for references, valid enum discriminants, valid UTF-8 in `str`, strict aliasing rule compliance, avoiding data races.

7. **How does Rust's memory safety model compare to manual memory management patterns in C (arenas, memory pools)?**
   * *Key aspects to address:* Custom allocator integration (`#[global_allocator]`), arena allocation libraries (`typed-arena`, `bumpalo`), region-based lifetimes matching arena lifecycles.
