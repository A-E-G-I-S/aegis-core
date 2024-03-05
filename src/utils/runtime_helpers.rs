use tokio::runtime::Runtime;
use tracing::{event, Level};

use super::Error;
use crate::types::other::aegis_error::AegisError;

pub trait RuntimeSpawnHandled {
  async fn spawn_handled<F, T>(&self, task: &str, future: F) -> Result<T, AegisError>
  where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static;
}

impl RuntimeSpawnHandled for Runtime {
  async fn spawn_handled<F, T>(&self, task: &str, future: F) -> Result<T, AegisError>
  where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
  {
    self
      .spawn(future)
      .await
      .map_or_else(|e| Err(AegisError::spawn(task, Box::new(e))), |f| Ok(f))
  }
}

impl AegisError {
  pub fn spawn(task: &str, e: Error) -> Self {
    Self::new(format!("Failed to complete task '{task}'"), Some(e))
  }
}
