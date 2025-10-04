use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use crate::services::storage::Storage;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMessage {
    pub job_id: String,
    pub job_type: String,
    pub media_location: String,
}

#[derive(Clone)]
pub struct Queue {
    sender: Sender<JobMessage>,
    statuses: Arc<Mutex<HashMap<String, JobStatus>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Processing { progress: u32 },
    Completed { result_url: String },
    Failed { error: String },
}

impl Queue {
    pub fn new(buffer: usize) -> (Self, Receiver<JobMessage>) {
        let (tx, rx) = channel(buffer);
        let statuses = Arc::new(Mutex::new(HashMap::new()));
        (
            Self { sender: tx, statuses },
            rx,
        )
    }

    pub async fn enqueue(&self, job: JobMessage) -> Result<(), ()> {
        // mark queued
        let mut s = self.statuses.lock().await;
        s.insert(job.job_id.clone(), JobStatus::Queued);
        drop(s);
        self.sender.send(job).await.map_err(|_| ())
    }

    pub async fn get_status(&self, job_id: &str) -> Option<JobStatus> {
        let s = self.statuses.lock().await;
        s.get(job_id).cloned()
    }

    pub fn get_statuses_handle(&self) -> Arc<Mutex<HashMap<String, JobStatus>>> {
        self.statuses.clone()
    }
}

pub fn start_worker_with_status(mut rx: Receiver<JobMessage>, storage: Arc<dyn Storage>, statuses: Arc<Mutex<HashMap<String, JobStatus>>>) {
    tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            tracing::info!("Worker picked job {} (type={})", job.job_id, job.job_type);
            // mark processing
            {
                let mut s = statuses.lock().await;
                s.insert(job.job_id.clone(), JobStatus::Processing { progress: 0 });
            }

            // Simulate processing with progress
            for p in (1..=5).map(|i| i * 20) {
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                let mut s = statuses.lock().await;
                s.insert(job.job_id.clone(), JobStatus::Processing { progress: p as u32 });
            }

            // Simulate result
            let content = format!("processed job {}", job.job_id).into_bytes();
            match storage.save_bytes(&content, &format!("result_{}.txt", job.job_id)) {
                Ok(loc) => {
                    let mut s = statuses.lock().await;
                    s.insert(job.job_id.clone(), JobStatus::Completed { result_url: loc });
                    tracing::info!("Job {} result saved", job.job_id);
                }
                Err(e) => {
                    let mut s = statuses.lock().await;
                    s.insert(job.job_id.clone(), JobStatus::Failed { error: format!("{:?}", e) });
                    tracing::error!("Failed to save result for {}: {:?}", job.job_id, e);
                }
            }
        }
        tracing::info!("Worker exiting - channel closed");
    });
}
