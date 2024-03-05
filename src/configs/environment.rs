use crate::utils::ExpectError;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref STDOUT_LOG_SEVERITY: String =
    std::env::var("STDOUT_LOG_SEVERITY").unwrap_or_else(|_| "INFO".to_string());
  pub static ref LOG_DIRECTORY: String =
    std::env::var("LOG_DIRECTORY").unwrap_or_else(|_| "/var/log/aegis".to_string());
}
