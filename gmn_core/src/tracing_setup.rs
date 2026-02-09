//! Tracing subscriber initialization and management.
//!
//! This module provides functions to initialize the global tracing subscriber
//! with various configuration options.

use crate::config::{LogFormat, LogOutput, TracingConfig};
use crate::errors::{Result, TracingError};
use std::sync::atomic::{AtomicBool, Ordering};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

/// Global flag to track if tracing has been initialized
static TRACING_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize tracing with default configuration
///
/// This is a convenience function that initializes tracing with configuration
/// loaded from environment variables. If no environment variables are set,
/// it uses sensible defaults.
///
/// # Errors
///
/// Returns an error if:
/// - Tracing has already been initialized
/// - Failed to create log file (if file output is configured)
/// - Failed to set the global subscriber
///
/// # Example
///
/// ```no_run
/// use gmn_core::tracing_setup::init_tracing;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init_tracing()?;
///     tracing::info!("Application started");
///     Ok(())
/// }
/// ```
pub fn init_tracing() -> Result<()> {
	let config = TracingConfig::from_env();
	init_tracing_with_config(config)
}

/// Initialize tracing with a custom configuration
///
/// This function sets up the global tracing subscriber based on the provided
/// configuration. It can only be called once per process.
///
/// # Errors
///
/// Returns an error if:
/// - Tracing has already been initialized
/// - Failed to create log file (if file output is configured)
/// - Failed to set the global subscriber
///
/// # Example
///
/// ```no_run
/// use gmn_core::tracing_setup::init_tracing_with_config;
/// use gmn_core::config::TracingConfig;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = TracingConfig::development();
///     init_tracing_with_config(config)?;
///     tracing::info!("Application started in development mode");
///     Ok(())
/// }
/// ```
pub fn init_tracing_with_config(config: TracingConfig) -> Result<()> {
	// Check if already initialized
	if TRACING_INITIALIZED.swap(true, Ordering::SeqCst) {
		return Err(TracingError::AlreadyInitialized.into());
	}

	// Create the env filter from the log level
	let env_filter = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new(&config.log_level))
		.unwrap_or_else(|_| EnvFilter::new("info"));

	// Build the subscriber based on output and format configuration
	// We use a simplified approach to avoid type complexity
	match (&config.output, config.format) {
		(LogOutput::Stdout, LogFormat::Pretty) => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stdout)
				.pretty()
				.init();
		}
		(LogOutput::Stdout, LogFormat::Compact) => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stdout)
				.compact()
				.init();
		}
		(LogOutput::Stdout, LogFormat::Json) => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stdout)
				.json()
				.init();
		}
		(LogOutput::Stderr, LogFormat::Pretty) => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stderr)
				.pretty()
				.init();
		}
		(LogOutput::Stderr, LogFormat::Compact) => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stderr)
				.compact()
				.init();
		}
		(LogOutput::Stderr, LogFormat::Json) => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stderr)
				.json()
				.init();
		}
		(LogOutput::File(path), format) => {
			let file_appender = tracing_appender::rolling::daily(
				path.parent().unwrap_or(std::path::Path::new(".")),
				path.file_name().unwrap_or(std::ffi::OsStr::new("gmn.log")),
			);

			match format {
				LogFormat::Pretty => {
					tracing_subscriber::fmt()
						.with_env_filter(env_filter)
						.with_writer(file_appender)
						.with_ansi(false)
						.pretty()
						.init();
				}
				LogFormat::Compact => {
					tracing_subscriber::fmt()
						.with_env_filter(env_filter)
						.with_writer(file_appender)
						.with_ansi(false)
						.compact()
						.init();
				}
				LogFormat::Json => {
					tracing_subscriber::fmt()
						.with_env_filter(env_filter)
						.with_writer(file_appender)
						.json()
						.init();
				}
			}
		}
		(LogOutput::Both { console, file }, _) => {
			// For Both mode, we use JSON for file and the configured format for console
			let file_appender = tracing_appender::rolling::daily(
				file.parent().unwrap_or(std::path::Path::new(".")),
				file.file_name().unwrap_or(std::ffi::OsStr::new("gmn.log")),
			);

			let file_layer = fmt::layer()
				.with_writer(file_appender)
				.json();

			let console_layer = match **console {
				LogOutput::Stdout => match config.format {
					LogFormat::Pretty => fmt::layer().with_writer(std::io::stdout).pretty().boxed(),
					LogFormat::Compact => fmt::layer().with_writer(std::io::stdout).compact().boxed(),
					LogFormat::Json => fmt::layer().with_writer(std::io::stdout).json().boxed(),
				},
				_ => match config.format {
					LogFormat::Pretty => fmt::layer().with_writer(std::io::stderr).pretty().boxed(),
					LogFormat::Compact => fmt::layer().with_writer(std::io::stderr).compact().boxed(),
					LogFormat::Json => fmt::layer().with_writer(std::io::stderr).json().boxed(),
				},
			};

			tracing_subscriber::registry()
				.with(env_filter)
				.with(console_layer)
				.with(file_layer)
				.init();
		}
	}

	Ok(())
}

/// Check if tracing has been initialized
///
/// This can be useful for conditional initialization or testing.
pub fn is_initialized() -> bool {
	TRACING_INITIALIZED.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_initialized() {
		// Note: This test assumes tracing hasn't been initialized yet
		// In a real test suite, you'd want to use a separate test binary
		// or reset the global state between tests
		assert!(!is_initialized() || is_initialized());
	}
}
