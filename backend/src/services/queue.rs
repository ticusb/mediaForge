// backend/src/services/queue.rs
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;

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
    // Optional redis connection manager. If present, enqueue will push to redis list
    redis: Option<ConnectionManager>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Processing { progress: u32 },
    Completed { result_url: String },
    Failed { error: String },
}

impl Queue {
    /// Create a new in-memory queue. If redis_url is Some, attempt to connect
    /// asynchronously and set up a connection manager; caller should be running
    /// inside a Tokio runtime and await this function.
    pub async fn new(buffer: usize, redis_url: Option<&str>) -> (Self, Receiver<JobMessage>) {
        let (tx, rx) = channel(buffer);
        let statuses = Arc::new(Mutex::new(HashMap::new()));

        let redis_conn = match redis_url {
            Some(url) => match redis::Client::open(url) {
                Ok(client) => match client.get_tokio_connection_manager().await {
                    Ok(cm) => Some(cm),
                    Err(e) => {
                        tracing::warn!("Failed to create redis connection manager: {:?}. Falling back to in-memory queue.", e);
                        None
                    }
                },
                Err(e) => {
                    tracing::warn!("Invalid redis url '{}': {:?}. Falling back to in-memory queue.", url, e);
                    None
                }
            },
            None => None,
        };

        (
            Self { sender: tx, statuses, redis: redis_conn },
            rx,
        )
    }

    pub async fn enqueue(&self, job: JobMessage) -> Result<(), ()> {
        // mark queued
        let mut s = self.statuses.lock().await;
        s.insert(job.job_id.clone(), JobStatus::Queued);
        drop(s);

        // If we have redis, push to list; otherwise use in-memory channel
        if let Some(conn_mgr) = &self.redis {
            let mut conn = conn_mgr.clone();
            let payload = serde_json::to_string(&job).map_err(|_| ())?;
            let push_res: Result<(), redis::RedisError> = async {
                let mut c = conn;
                c.rpush("mediaforge:job_queue", payload).await.map(|_: i64| ())
            }
            .await;

            match push_res {
                Ok(_) => Ok(()),
                Err(e) => {
                    tracing::warn!("Redis enqueue failed: {:?} - falling back to local channel", e);
                    self.sender.send(job).await.map_err(|_| ())
                }
            }
        } else {
            self.sender.send(job).await.map_err(|_| ())
        }
    }

    pub async fn get_status(&self, job_id: &str) -> Option<JobStatus> {
        let s = self.statuses.lock().await;
        s.get(job_id).cloned()
    }

    pub fn get_statuses_handle(&self) -> Arc<Mutex<HashMap<String, JobStatus>>> {
        self.statuses.clone()
    }

    /// Forward a job to the local in-process channel. Used by redis poller to
    /// insert jobs into the worker channel.
    pub async fn forward_to_local(&self, job: JobMessage) -> Result<(), ()> {
        self.sender.send(job).await.map_err(|_| ())
    }
}