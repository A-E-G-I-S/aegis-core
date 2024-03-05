use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use tracing::{event, Level};

use crate::utils::Error as OurErr;

#[derive(Debug)]
pub struct AegisError {
  pub message: String,
  pub source: Option<OurErr>,
}

impl fmt::Display for AegisError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = &self.message;
    if let Some(source) = &self.source {
      write!(f, "{message} Source Error: {source}")
    } else {
      write!(f, "{message}")
    }
  }
}

impl Error for AegisError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.source.as_deref().map(|s| s as _)
  }
}

impl AegisError {
  pub fn new(message: String, source: Option<OurErr>) -> Self {
    let result = Self { message, source };
    event!(Level::ERROR, "{result}");
    result
  }
}
