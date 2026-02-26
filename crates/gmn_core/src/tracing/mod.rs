//! Tracing and Logging Module
//!
//! This module provides comprehensive observability with structured logging,
//! tracing configuration, and instrumentation utilities.

pub mod config;
pub mod instrumentation;
pub mod setup;

// Re-exports for convenience
pub use config::TracingConfig;
pub use setup::{init_tracing, init_tracing_with_config};
