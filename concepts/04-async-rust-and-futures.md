# Topic 04: Async Rust & Futures

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What is a `Future` in Rust? What does it mean that futures are "lazy"?**
   * *Key aspects to address:* `Poll::Ready(T)` vs `Poll::Pending`, execution drive requirement, zero-cost state machine generation, futures doing no work until `poll()` is explicitly invoked by an executor.

2. **What is future bloating, and why do deeply nested async call chains cause code size / stack size issues? How would you address it?**
   * *Key aspects to address:* Compiler-generated anonymous enum state machines for every `async fn`, aggregate size explosion on large stack frames across `.await` points, mitigation using `Box::pin(async move { ... })`.

3. **Walk through how an `async fn` desugars into a state machine.**
   * *Key aspects to address:* `Future` trait implementation, state representation across `.await` boundaries, holding local variables in enum variants, stack layout transition.

4. **What is `Pin`, and why does async Rust need it?**
   * *Key aspects to address:* Self-referential structs created by `async` blocks (holding references to variables across `.await` points), preventing pointer invalidation due to moves in memory, `Unpin` marker trait.

5. **What's the difference between `Send` and `Sync`? Why do these matter for async tasks specifically?**
   * *Key aspects to address:* `Send` (safe to transfer ownership across thread boundary) vs `Sync` (safe to share immutable references `&T` across threads). Work-stealing async runtimes requiring futures spawned across tasks to be `Send`.

6. **What is a runtime (e.g., Tokio), and why doesn't Rust ship one in `std`?**
   * *Key aspects to address:* Reactor/Executor architecture, I/O multiplexing (`epoll`/`kqueue`/`IOCP`), keeping `std` lightweight and runtime-agnostic for embedded, real-time, and kernel environments.

7. **What happens if you call a blocking operation inside an async task? How do you avoid it (`spawn_blocking`, etc.)?**
   * *Key aspects to address:* Worker thread starvation in fixed-size thread pools, event loop lag, delegating synchronous/blocking CPU work to `tokio::task::spawn_blocking` or dedicated OS thread pools.

8. **What's the difference between `tokio::spawn` and directly `.await`-ing a future?**
   * *Key aspects to address:* Task allocation on executor work-stealing queue vs inline evaluation on current task stack, concurrent background execution vs sequential continuation.

9. **What is cooperative scheduling / task yielding, and why can a poorly written async task starve others?**
   * *Key aspects to address:* Non-preemptive task execution model, futures yielding control back to executor at `.await` boundaries, tight CPU loops missing `.await` points, `tokio::task::yield_now()`.

10. **What's the difference between `join!`, `select!`, and spawning separate tasks?**
    * *Key aspects to address:* Parallel concurrent polling on single thread/task vs race completion / cancellation vs independent execution across thread pool.

11. **What is backpressure, and how do bounded channels help implement it?**
    * *Key aspects to address:* Flow control, preventing unbounded memory growth when producer outpaces consumer, producer blocking/awaiting when channel capacity limit is reached.

12. **How does cancellation work in async Rust? What happens to a task's state when it's dropped mid-await?**
    * *Key aspects to address:* Drop-based cancellation semantics, future destruction without running further code, RAII cleanup guarantees, resource leaks prevention, cancellation safety in `tokio::select!`.
