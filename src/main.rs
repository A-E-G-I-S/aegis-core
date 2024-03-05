#![forbid(unsafe_code)]
#![deny(warnings)]
#![allow(unused)] // Temporarily here while we are working on the project
#![warn(
  clippy::complexity,
  clippy::pedantic,
  clippy::nursery,
  clippy::suspicious,
  clippy::perf,
  clippy::unwrap_used
)]

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::SystemTime;
use tracing::{event, Level};

use crate::configs::environment::{LOG_DIRECTORY, STDOUT_LOG_SEVERITY};
use crate::utils::ExpectError;

mod configs;
mod modules;
mod types;
mod utils;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
  let time: DateTime<Utc> = SystemTime::now().into();
  let time_str = time.to_rfc3339();

  // .env
  dotenv().expect_error(|e| format!("Failed to load .env file: {e}"));

  // Logging
  std::env::set_var("RUST_SPANTRACE", "1");
  std::env::set_var("RUST_BACKTRACE", "1");
  std::env::set_var("RUST_LIB_BACKTRACE", "full");
  color_eyre::install().unwrap_or_default();

  let level = match STDOUT_LOG_SEVERITY.as_str() {
    "TRACE" => Level::TRACE,
    "DEBUG" => Level::DEBUG,
    "WARN" => Level::WARN,
    "ERROR" => Level::ERROR,
    _ => Level::INFO,
  };
  let file_appender0 = tracing_appender::rolling::never(&*LOG_DIRECTORY, format!("{time_str}.log"));
  let (non_blocking0, _guard) = tracing_appender::non_blocking(file_appender0);
  tracing_subscriber::fmt()
    .with_max_level(level)
    .with_writer(non_blocking0)
    .init();

  event!(Level::INFO, "Starting AEGIS...");

  // Do Startup routines
}
