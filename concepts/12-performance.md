# Topic 12: Performance

[← Back to Concepts Index](./README.md)

---

### Questions

1. **How would you profile and benchmark Rust code? What role does Criterion play?**
   * *Key aspects to address:* Micro-benchmarking with `criterion` (statistical analysis, outlier detection, regression benchmarks), profiling tools (`perf`, `flamegraph`, `samply`, Valgrind/Callgrind).

2. **What's the difference between parallel I/O and streaming I/O, and when is each preferable?**
   * *Key aspects to address:* Concurrent chunks processing vs sequential memory-bounded stream processing (`AsyncRead`/`AsyncWrite`/`Stream`), throughput vs memory footprint trade-offs.

3. **How would you diagnose and fix false sharing in a concurrent data structure?**
   * *Key aspects to address:* Identifying cache invalidation bottlenecks via `perf c2c`, adding explicit alignment padding (`crossbeam-utils::CachePadded` or `#[repr(align(64))]`).

4. **What are the performance implications of `Arc<Mutex<HashMap>>` under read-heavy load, and how would you improve it?**
   * *Key aspects to address:* Lock contention bottleneck, reader thread serialization. Improvements: `Arc<RwLock<HashMap>>`, sharded maps (`DashMap`), lock-free read structures (RCU / `arc-swap`).

5. **What's the difference between stack and heap allocation in Rust, and how does that affect performance?**
   * *Key aspects to address:* Continuous L1/L2 cache pointer arithmetic vs allocator syscall indirection (`malloc`/`jemalloc`), fragmentation, lifetime bounds, escaping stack frames.

6. **When does zero-cost abstraction actually hold, and when does it break down in practice?**
   * *Key aspects to address:* LLVM optimization passes, loop unrolling, monomorphization inlining vs binary bloat, instruction cache (I-cache) thrashing, non-inlined vtable dynamic dispatch indirection.
