# Topic 07: Traits & Generics

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What's the difference between a trait object (`dyn Trait`) and a generic with a trait bound (`impl Trait` / `<T: Trait>`)?**
   * *Key aspects to address:* Monomorphization & static dispatch (zero runtime cost, code bloat) vs Fat pointers & dynamic dispatch (vtable lookup runtime cost, heterogeneous collections).

2. **What is static dispatch vs dynamic dispatch, and what are the tradeoffs?**
   * *Key aspects to address:* Compile-time specialized function generation vs runtime indirection via virtual tables (`vtable`), inlining capability vs binary size and cache performance.

3. **What is a vtable, and how does Rust implement dynamic dispatch under the hood?**
   * *Key aspects to address:* Fat pointer structure (data pointer + vtable pointer), vtable layout (destructor `drop_in_place`, size, alignment, method function pointers).

4. **What are trait objects' restrictions (object safety)? Why can't all traits become `dyn Trait`?**
   * *Key aspects to address:* Object safety rules (no `Self: Sized` requirement, no generic methods, no static functions without receiver `self`), inability to build fixed vtable when signatures depend on concrete `Self`.

5. **What's the difference between associated types and generic type parameters on a trait?**
   * *Key aspects to address:* Single implementation per type vs multiple implementations per generic type parameter (`Iterator::Item` vs `From<T>`).

6. **What is the `Deref`/`DerefMut` trait, and what is "deref coercion"?**
   * *Key aspects to address:* Transparent method delegation, implicit conversion from `&T` to `&U` when `T: Deref<Target = U>`, avoiding manual indirection (`String` -> `&str`, `Box<T>` -> `&T`).

7. **What's the difference between `From`/`Into` and `TryFrom`/`TryInto`?**
   * *Key aspects to address:* Infallible type conversion (blanket `Into` implementation) vs fallible conversion returning `Result<T, Error>`.

8. **What is the newtype pattern, and why is it commonly used in Rust?**
   * *Key aspects to address:* Zero-cost single-field tuple wrapper (`struct Meter(u64)`), bypassing the orphan rule for trait implementations, compile-time type safety enforcement (distinguishing IDs, units).
