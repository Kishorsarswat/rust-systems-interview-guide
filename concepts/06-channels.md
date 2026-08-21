# Topic 06: Channels

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What are channels in Rust, and what problem do they solve?**
   * *Key aspects to address:* Inter-thread / inter-task communication primitives ("Do not communicate by sharing memory; share memory by communicating"), ownership transfer across boundaries.

2. **What's the difference between `std::sync::mpsc` and `tokio::sync::mpsc`?**
   * *Key aspects to address:* Synchronous thread-blocking send/receive operations vs async task-awaiting `.send().await` and `.recv().await` methods without thread blocking.

3. **What's the difference between bounded and unbounded channels? What are the risks of unbounded channels?**
   * *Key aspects to address:* Fixed buffer size enforcing backpressure vs infinite allocation buffer risk under high traffic load leading to Out-Of-Memory (OOM) crashes.

4. **What is a `oneshot` channel, and when would you use one?**
   * *Key aspects to address:* Single producer, single consumer, exactly one message payload. Use cases: async request-response pairing, RPC return values, signal cancellation.

5. **What's the difference between `mpsc`, `broadcast`, and `watch` channels in Tokio?**
   * *Key aspects to address:* Multi-producer single-consumer vs multi-producer multi-consumer publish-subscribe (every consumer sees all messages) vs single-value state change notifications (latest value state observation).

6. **How would you implement a producer-consumer pipeline using channels, and how would backpressure come into play?**
   * *Key aspects to address:* Chained processing stages using bounded `mpsc` channels, worker scaling, buffer depth balancing, natural upstream throttling when downstream slows down.
