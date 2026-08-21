//! Reference Solution: Async Token Bucket Rate Limiter

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct RateLimiterSol {
    inner: Arc<Mutex<RateLimiterInner>>,
}

struct RateLimiterInner {
    max_tokens: f64,
    refill_rate_per_sec: f64,
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiterInner {
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate_per_sec;

        if new_tokens > 0.0 {
            self.tokens = (self.tokens + new_tokens).min(self.max_tokens);
            self.last_refill = now;
        }
    }
}

impl RateLimiterSol {
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

    pub async fn try_acquire(&self, tokens: usize) -> bool {
        let mut inner = self.inner.lock().await;
        inner.refill();

        let req = tokens as f64;
        if inner.tokens >= req {
            inner.tokens -= req;
            true
        } else {
            false
        }
    }

    pub async fn acquire(&self, tokens: usize) {
        let req = tokens as f64;
        loop {
            let mut inner = self.inner.lock().await;
            inner.refill();

            if inner.tokens >= req {
                inner.tokens -= req;
                return;
            }

            let missing = req - inner.tokens;
            let wait_secs = missing / inner.refill_rate_per_sec;
            drop(inner);

            tokio::time::sleep(Duration::from_secs_f64(wait_secs.max(0.001))).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_rate_limiter() {
        let limiter = RateLimiterSol::new(2, 10);

        assert!(limiter.try_acquire(1).await);
        assert!(limiter.try_acquire(1).await);
        assert!(!limiter.try_acquire(1).await); // Empty

        limiter.acquire(1).await; // Refills quickly
    }
}
