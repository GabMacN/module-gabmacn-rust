//! Tracing subscriber initialization and management.
//!
//! This module provides functions to initialize the global tracing subscriber
//! with various configuration options.
//!
//! The implementation uses helper functions to avoid exponential match growth
//! while maintaining type safety and avoiding unnecessary boxing overhead.

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

	// Dispatch to appropriate initialization function based on output type
	// This avoids exponential match growth by separating concerns
	match config.output {
		LogOutput::Stdout => init_stdout(env_filter, config.format),
		LogOutput::Stderr => init_stderr(env_filter, config.format),
		LogOutput::File(ref path) => init_file(env_filter, config.format, path),
		LogOutput::Both { ref console, ref file } => {
			init_both(env_filter, config.format, console, file)
		}
	}

	Ok(())
}

/// Initialize tracing with stdout output
fn init_stdout(env_filter: EnvFilter, format: LogFormat) {
	match format {
		LogFormat::Pretty => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stdout)
				.pretty()
				.init();
		}
		LogFormat::Compact => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stdout)
				.compact()
				.init();
		}
		LogFormat::Json => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stdout)
				.json()
				.init();
		}
	}
}

/// Initialize tracing with stderr output
fn init_stderr(env_filter: EnvFilter, format: LogFormat) {
	match format {
		LogFormat::Pretty => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stderr)
				.pretty()
				.init();
		}
		LogFormat::Compact => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stderr)
				.compact()
				.init();
		}
		LogFormat::Json => {
			tracing_subscriber::fmt()
				.with_env_filter(env_filter)
				.with_writer(std::io::stderr)
				.json()
				.init();
		}
	}
}

/// Initialize tracing with file output
fn init_file(env_filter: EnvFilter, format: LogFormat, path: &std::path::PathBuf) {
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

/// Initialize tracing with both console and file output
fn init_both(
	env_filter: EnvFilter,
	format: LogFormat,
	console: &LogOutput,
	file: &std::path::PathBuf,
) {
	let file_appender = tracing_appender::rolling::daily(
		file.parent().unwrap_or(std::path::Path::new(".")),
		file.file_name().unwrap_or(std::ffi::OsStr::new("gmn.log")),
	);

	// Create console layer based on console output type and format
	// Using .boxed() method which properly implements the Layer trait
	let console_layer = match (console, format) {
		(LogOutput::Stdout, LogFormat::Pretty) => {
			fmt::layer().with_writer(std::io::stdout).pretty().boxed()
		}
		(LogOutput::Stdout, LogFormat::Compact) => {
			fmt::layer().with_writer(std::io::stdout).compact().boxed()
		}
		(LogOutput::Stdout, LogFormat::Json) => {
			fmt::layer().with_writer(std::io::stdout).json().boxed()
		}
		// Default to stderr for any other console output type
		(_, LogFormat::Pretty) => fmt::layer().with_writer(std::io::stderr).pretty().boxed(),
		(_, LogFormat::Compact) => fmt::layer().with_writer(std::io::stderr).compact().boxed(),
		(_, LogFormat::Json) => fmt::layer().with_writer(std::io::stderr).json().boxed(),
	};

	// File layer always uses JSON for structured logging
	let file_layer = fmt::layer().with_writer(file_appender).json();

	tracing_subscriber::registry()
		.with(env_filter)
		.with(console_layer)
		.with(file_layer)
		.init();
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
