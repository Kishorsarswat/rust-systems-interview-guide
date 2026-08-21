# Topic 05: Concurrency Models: Threads vs Tasks

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What's the difference between an OS thread and an async task?**
   * *Key aspects to address:* Kernel-managed thread stacks (MBs) & context switches vs user-space green tasks (Bytes) scheduled by runtime reactor loop, stackless state machines.

2. **When would you reach for `std::thread` vs `tokio::spawn` vs `rayon`?**
   * *Key aspects to address:* Long-running dedicated OS threads vs lightweight I/O-bound concurrency vs CPU-bound data-parallelism work-stealing algorithms.

3. **What is Rayon, and how does it differ from Tokio's model (data parallelism vs I/O concurrency)?**
   * *Key aspects to address:* Work-stealing CPU thread pool, `ParallelIterator`, divide-and-conquer processing vs event-driven non-blocking I/O multiplexing.

4. **How do you share state safely across threads? Across async tasks?**
   * *Key aspects to address:* `Arc<Mutex<T>>`, `Arc<RwLock<T>>`, atomics vs channels / message passing vs `tokio::sync::Mutex` across `.await` points.

5. **What is the actor model, and how might you implement something like it in Rust?**
   * *Key aspects to address:* Private encapsulated state, mailbox channels (`mpsc`), processing loop handling incoming messages sequentially, avoiding shared mutable state locks.
