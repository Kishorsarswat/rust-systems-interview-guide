# Topic 02: Smart Pointers & Interior Mutability

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What's the difference between `Box<T>`, `Rc<T>`, and `Arc<T>`?**
   * *Key aspects to address:* Single owner heap allocation vs reference-counted shared ownership vs thread-safe atomic reference counting. Memory layouts and runtime trade-offs.

2. **What is interior mutability, and why is it needed given Rust's borrowing rules?**
   * *Key aspects to address:* Mutating data through an immutable reference (`&T`), bypassing compile-time borrow checker checks in exchange for runtime safety invariants (`UnsafeCell<T>`).

3. **Compare `RefCell<T>` vs `Mutex<T>` vs `RwLock<T>`. When would you pick each?**
   * *Key aspects to address:* Single-threaded dynamic borrowing (`RefCell`) vs multi-threaded mutual exclusion (`Mutex`) vs shared-read/exclusive-write locking (`RwLock`).

4. **What happens if you violate `RefCell`'s borrow rules at runtime?**
   * *Key aspects to address:* Dynamic counter tracking active borrows, triggering a runtime `panic!` upon simultaneous `borrow_mut()` and `borrow()`.

5. **What is `Cell<T>`, and how is it different from `RefCell<T>`?**
   * *Key aspects to address:* Value-copy/move semantics vs reference borrowing, zero overhead, no borrow guards, requirement for `Copy` or replacement operations (`Cell::replace`, `Cell::set`).

6. **Why does `Rc` require `RefCell` (or similar) for mutation, but `Arc` typically pairs with `Mutex` or `RwLock`?**
   * *Key aspects to address:* Thread safety traits (`Send`/`Sync`), non-atomic operations in `Rc`/`RefCell` causing data races across threads vs atomic synchronization in `Arc` combined with thread-safe locking.

7. **What is a weak reference (`Weak<T>`), and what problem does it solve?**
   * *Key aspects to address:* Strong vs weak reference counts, breaking reference cycles (cyclic data structures like graphs or trees with parent pointers), memory leaks prevention.
