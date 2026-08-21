//! Reference Solution: Bounded Async Worker Pool

use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct WorkerPoolSol {
    sender: mpsc::Sender<Job>,
}

impl WorkerPoolSol {
    pub fn new(num_workers: usize, queue_capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>(queue_capacity);
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..num_workers {
            let rx = Arc::clone(&receiver);
            tokio::spawn(async move {
                loop {
                    let mut lock = rx.lock().await;
                    let job = lock.recv().await;
                    drop(lock);

                    match job {
                        Some(task) => task(),
                        None => break,
                    }
                }
            });
        }

        Self { sender }
    }

    pub async fn submit<F, T>(&self, f: F) -> Result<oneshot::Receiver<T>, &'static str>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let job = Box::new(move || {
            let res = f();
            let _ = tx.send(res);
        });

        self.sender
            .send(job)
            .await
            .map_err(|_| "Worker pool queue is closed")?;

        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_worker_pool() {
        let pool = WorkerPoolSol::new(2, 10);

        let rx1 = pool.submit(|| 10 + 20).await.unwrap();
        let rx2 = pool.submit(|| "hello".to_uppercase()).await.unwrap();

        assert_eq!(rx1.await.unwrap(), 30);
        assert_eq!(rx2.await.unwrap(), "HELLO");
    }
}
