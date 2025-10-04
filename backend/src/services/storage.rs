use std::path::{PathBuf};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
}

pub trait Storage: Send + Sync {
    fn save_bytes(&self, bytes: &[u8], filename_hint: &str) -> Result<String, StorageError>;
}

pub struct LocalStorage {
    pub base_path: PathBuf,
}

impl LocalStorage {
    pub fn new<P: Into<PathBuf>>(base: P) -> Self {
        Self { base_path: base.into() }
    }
}

impl Storage for LocalStorage {
    fn save_bytes(&self, bytes: &[u8], filename_hint: &str) -> Result<String, StorageError> {
        let id = Uuid::new_v4().to_string();
        let filename = format!("{}_{}", id, filename_hint);
        let mut path = self.base_path.clone();
        std::fs::create_dir_all(&path).map_err(StorageError::Io)?;
        path.push(filename);
        let mut f = File::create(&path).map_err(StorageError::Io)?;
        f.write_all(bytes).map_err(StorageError::Io)?;
        Ok(path.to_string_lossy().to_string())
    }
}

// Placeholder for S3/MinIO implementation
pub struct S3Storage {
    pub bucket: String,
    pub endpoint: String,
}

impl S3Storage {
    pub fn new(bucket: &str, endpoint: &str) -> Self {
        Self { bucket: bucket.to_string(), endpoint: endpoint.to_string() }
    }
}

impl Storage for S3Storage {
    fn save_bytes(&self, _bytes: &[u8], _filename_hint: &str) -> Result<String, StorageError> {
        // Not implemented in MVP scaffolding; integrate rusoto/s3 or aws-sdk-s3 later.
        Err(StorageError::Io(std::io::Error::new(std::io::ErrorKind::Other, "S3 storage not implemented")))
    }
}
