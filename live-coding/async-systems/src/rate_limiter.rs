//! # Challenge: Async Token Bucket Rate Limiter
//!
//! ## Problem Statement
//! Design and implement a thread-safe, async Token Bucket Rate Limiter using Tokio.
//!
//! The rate limiter maintains a bucket of up to `max_tokens`. Tokens are refilled at a continuous rate
//! of `refill_rate_per_sec`.
//!
//! Support the following operations:
//! * `try_acquire(&self, tokens: usize) -> bool`: Non-blocking check. If enough tokens are available,
//!   deducts them immediately and returns `true`. Otherwise returns `false`.
//! * `acquire(&self, tokens: usize) -> impl Future<Output = ()>`: Asynchronously waits (sleeps) until
//!   the required number of tokens become available, then consumes them.

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct RateLimiter {
    inner: Arc<Mutex<RateLimiterInner>>,
}

struct RateLimiterInner {
    max_tokens: f64,
    refill_rate_per_sec: f64,
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    /// Creates a new `RateLimiter`.
    pub fn new(max_tokens: usize, refill_rate_per_sec: usize) -> Self {
        let max_tokens = max_tokens as f64;
        let refill_rate_per_sec = refill_rate_per_sec as f64;
        Self {
            inner: Arc::new(Mutex::new(RateLimiterInner {
                max_tokens,
                refill_rate_per_sec,
                tokens: max_tokens,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Attempts to acquire `tokens` without blocking/sleeping. Returns `true` if acquired.
    pub async fn try_acquire(&self, _tokens: usize) -> bool {
        todo!("Implement RateLimiter::try_acquire(&self, tokens)")
    }

    /// Waits asynchronously until `tokens` are available, consuming them.
    pub async fn acquire(&self, _tokens: usize) {
        todo!("Implement RateLimiter::acquire(&self, tokens)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Implement RateLimiter::try_acquire")]
    async fn test_starter_rate_limiter_panics_todo() {
        let limiter = RateLimiter::new(10, 1);
        limiter.try_acquire(1).await;
    }

    #[tokio::test]
    #[ignore] // Remove #[ignore] once you implement try_acquire and acquire!
    async fn test_rate_limiter_basic_try_acquire() {
        let limiter = RateLimiter::new(2, 1);

        assert!(limiter.try_acquire(1).await);
        assert!(limiter.try_acquire(1).await);
        assert!(!limiter.try_acquire(1).await); // Bucket empty
    }
}
