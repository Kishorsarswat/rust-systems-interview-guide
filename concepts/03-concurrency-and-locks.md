# Topic 03: Concurrency & Locks

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What are all the lock types available in Rust's ecosystem (`std`, `tokio`, `parking_lot`, etc.)?**
   * *Key aspects to address:* OS-backed standard locks vs user-space adaptive spinlocks (`parking_lot`) vs async non-blocking task locks (`tokio::sync`).

2. **What's the difference between `std::sync::Mutex`, `tokio::sync::Mutex`, and `parking_lot::Mutex`?**
   * *Key aspects to address:* Blocking thread vs yields task `.await` point, memory overhead (1 byte guard in `parking_lot` vs OS mutex overhead), lock guard holding across yield points.

3. **Why is it dangerous to hold a `std::sync::Mutex` guard across an `.await` point?**
   * *Key aspects to address:* Guard is non-`Send` (or keeps lock held while thread context switches), thread starvation, deadlock risk when Tokio worker thread switches tasks while holding blocking lock.

4. **What is a `Condvar`, and what problem does it solve alongside a `Mutex`?**
   * *Key aspects to address:* Condition variables, avoiding busy-wait CPU spinning, atomically releasing mutex while parking thread until signaled (`notify_one`/`notify_all`).

5. **What's the difference between a `Mutex` and an `RwLock`? When does `RwLock` actually help vs hurt?**
   * *Key aspects to address:* Multiple readers vs single writer. Cache invalidation overhead on reader count updates, write starvation, reader lock contention under high core count.

6. **What is lock poisoning, and how does Rust's `std` handle it?**
   * *Key aspects to address:* Thread panicking while holding a `MutexGuard`, leaving state potentially inconsistent; `lock()` returning `Err(PoisonError)`; accessing dirty data via `into_inner()`.

7. **How would you design a data structure to minimize lock contention under read-heavy load?**
   * *Key aspects to address:* Read-Copy-Update (RCU), lock-free atomic pointer swaps, sharded locking (e.g., `DashMap`), lock-free atomic skips/queues.

8. **What is false sharing, and how does it relate to CPU cache lines?**
   * *Key aspects to address:* Independent variables residing on the same 64-byte L1/L2 cache line modified by separate CPU cores, triggering cache coherence protocol invalidation bounces (`Cacheline` alignment padding `#[repr(align(64))]`).

9. **What's the difference between a spinlock and a blocking lock? When would you choose `parking_lot` over `std`?**
   * *Key aspects to address:* Busy-looping CPU instructions vs OS thread parking/context-switching. Performance trade-offs for ultra-short critical sections vs long wait times.

10. **What are atomics (`AtomicUsize`, `AtomicBool`, etc.), and when would you reach for them instead of a lock?**
    * *Key aspects to address:* Hardware-level atomic primitives (CMPXCHG, LL/SC), zero lock acquisition overhead, lock-free state updates, hardware memory model guarantees.

11. **What are the different memory orderings (`Relaxed`, `Acquire`, `Release`, `SeqCst`), and when does the choice matter?**
    * *Key aspects to address:* CPU instruction reordering, acquire-release synchronizes-with relationship, `Relaxed` atomicity without ordering, `SeqCst` global total ordering.
