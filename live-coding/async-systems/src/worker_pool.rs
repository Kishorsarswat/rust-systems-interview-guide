//! # Challenge: Bounded Async Worker Pool with Graceful Shutdown
//!
//! ## Problem Statement
//! Design and implement an Async Task Worker Pool using Tokio channels (`mpsc` and `oneshot`).
//!
//! The worker pool spawns a fixed number of background worker tasks. Clients submit CPU or I/O
//! closures to the pool and receive a `oneshot::Receiver` to await the computed result asynchronously.
//!
//! Support the following operations:
//! * `submit<F, T>(&self, f: F) -> tokio::sync::oneshot::Receiver<T>`: Submits closure `f` to the pool.
//! * Graceful worker termination on channel drop or explicit shutdown.

use tokio::sync::{mpsc, oneshot};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct WorkerPool {
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
    /// Creates and starts a `WorkerPool` with `num_workers` concurrent background tasks.
    pub fn new(num_workers: usize, queue_capacity: usize) -> Self {
        let (sender, mut receiver) = mpsc::channel::<Job>(queue_capacity);

        // Convert receiver into a shared receiver or distribute across tasks
        let receiver = std::sync::Arc::new(tokio::sync::Mutex::new(receiver));

        for _ in 0..num_workers {
            let rx = Arc::clone(&receiver);
            tokio::spawn(async move {
                loop {
                    let mut lock = rx.lock().await;
                    let job = lock.recv().await;
                    drop(lock);

                    match job {
                        Some(task) => task(),
                        None => break, // Channel closed, shutdown worker
                    }
                }
            });
        }

        Self { sender }
    }

    /// Submits a task to the pool, returning a `oneshot::Receiver` for the task result.
    pub async fn submit<F, T>(&self, _f: F) -> Result<oneshot::Receiver<T>, &'static str>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        todo!("Implement WorkerPool::submit")
    }
}

use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Implement WorkerPool::submit")]
    async fn test_starter_worker_pool_panics_todo() {
        let pool = WorkerPool::new(2, 10);
        let _ = pool.submit(|| 42).await;
    }

    #[tokio::test]
    #[ignore] // Remove #[ignore] once you implement submit!
    async fn test_worker_pool_task_execution() {
        let pool = WorkerPool::new(2, 10);

        let rx1 = pool.submit(|| 10 + 20).await.unwrap();
        let rx2 = pool.submit(|| "hello".to_uppercase()).await.unwrap();

        assert_eq!(rx1.await.unwrap(), 30);
        assert_eq!(rx2.await.unwrap(), "HELLO");
    }
}
