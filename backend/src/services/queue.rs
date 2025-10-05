// backend/src/services/queue.rs
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMessage {
    pub job_id: String,
    pub user_id: String,
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