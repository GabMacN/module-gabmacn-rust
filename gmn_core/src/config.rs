//! Configuration module for tracing and logging behavior.
//!
//! This module provides configuration options for the tracing infrastructure,
//! including log levels, output formats, and environment variable support.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Output format for tracing logs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogFormat {
	/// Pretty-printed format with colors (best for development)
	Pretty,
	/// Compact format (minimal output)
	Compact,
	/// JSON format (best for production/structured logging)
	Json,
}

impl Default for LogFormat {
	fn default() -> Self {
		Self::Pretty
	}
}

impl LogFormat {
	/// Parse log format from string (case-insensitive)
	pub fn from_str(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"pretty" => Some(Self::Pretty),
			"compact" => Some(Self::Compact),
			"json" => Some(Self::Json),
			_ => None,
		}
	}
}

/// Output target for tracing logs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogOutput {
	/// Write to stdout
	Stdout,
	/// Write to stderr
	Stderr,
	/// Write to a file
	File(PathBuf),
	/// Write to both console and file
	Both {
		/// Console output (stdout or stderr)
		console: Box<LogOutput>,
		/// File path for persistent logs
		file: PathBuf,
	},
}

impl Default for LogOutput {
	fn default() -> Self {
		Self::Stderr
	}
}

/// Configuration for tracing and logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
	/// Log level filter (e.g., "debug", "info", "warn", "error")
	/// Can also use directive syntax like "gmn_core=debug,hyper=info"
	pub log_level: String,

	/// Output format
	pub format: LogFormat,

	/// Output target
	pub output: LogOutput,

	/// Whether to include timestamps
	pub with_timestamps: bool,

	/// Whether to include thread IDs
	pub with_thread_ids: bool,

	/// Whether to include thread names
	pub with_thread_names: bool,

	/// Whether to include file/line information
	pub with_file_line: bool,

	/// Whether to include span information
	pub with_span_list: bool,

	/// Whether to use ANSI colors (only applies to Pretty format)
	pub with_ansi: bool,
}

impl Default for TracingConfig {
	fn default() -> Self {
		Self {
			log_level: "info".to_string(),
			format: LogFormat::default(),
			output: LogOutput::default(),
			with_timestamps: true,
			with_thread_ids: false,
			with_thread_names: false,
			with_file_line: true,
			with_span_list: true,
			with_ansi: true,
		}
	}
}

impl TracingConfig {
	/// Create a new configuration with default values
	pub fn new() -> Self {
		Self::default()
	}

	/// Create configuration from environment variables
	///
	/// Supported environment variables:
	/// - `GMN_LOG_LEVEL`: Log level filter (default: "info")
	/// - `GMN_LOG_FORMAT`: Output format - "pretty", "compact", or "json" (default: "pretty")
	/// - `GMN_LOG_OUTPUT`: Output target - "stdout", "stderr", or file path (default: "stderr")
	/// - `GMN_LOG_TIMESTAMPS`: Include timestamps - "true" or "false" (default: "true")
	/// - `GMN_LOG_THREAD_IDS`: Include thread IDs - "true" or "false" (default: "false")
	/// - `GMN_LOG_THREAD_NAMES`: Include thread names - "true" or "false" (default: "false")
	/// - `GMN_LOG_FILE_LINE`: Include file/line info - "true" or "false" (default: "true")
	/// - `GMN_LOG_SPAN_LIST`: Include span list - "true" or "false" (default: "true")
	/// - `GMN_LOG_ANSI`: Use ANSI colors - "true" or "false" (default: "true")
	pub fn from_env() -> Self {
		let mut config = Self::default();

		if let Ok(level) = std::env::var("GMN_LOG_LEVEL") {
			config.log_level = level;
		}

		if let Ok(format) = std::env::var("GMN_LOG_FORMAT") {
			if let Some(fmt) = LogFormat::from_str(&format) {
				config.format = fmt;
			}
		}

		if let Ok(output) = std::env::var("GMN_LOG_OUTPUT") {
			config.output = match output.to_lowercase().as_str() {
				"stdout" => LogOutput::Stdout,
				"stderr" => LogOutput::Stderr,
				path => LogOutput::File(PathBuf::from(path)),
			};
		}

		if let Ok(val) = std::env::var("GMN_LOG_TIMESTAMPS") {
			config.with_timestamps = val.to_lowercase() == "true";
		}

		if let Ok(val) = std::env::var("GMN_LOG_THREAD_IDS") {
			config.with_thread_ids = val.to_lowercase() == "true";
		}

		if let Ok(val) = std::env::var("GMN_LOG_THREAD_NAMES") {
			config.with_thread_names = val.to_lowercase() == "true";
		}

		if let Ok(val) = std::env::var("GMN_LOG_FILE_LINE") {
			config.with_file_line = val.to_lowercase() == "true";
		}

		if let Ok(val) = std::env::var("GMN_LOG_SPAN_LIST") {
			config.with_span_list = val.to_lowercase() == "true";
		}

		if let Ok(val) = std::env::var("GMN_LOG_ANSI") {
			config.with_ansi = val.to_lowercase() == "true";
		}

		config
	}

	/// Create a preset configuration for development
	pub fn development() -> Self {
		Self {
			log_level: "debug".to_string(),
			format: LogFormat::Pretty,
			output: LogOutput::Stderr,
			with_timestamps: true,
			with_thread_ids: false,
			with_thread_names: false,
			with_file_line: true,
			with_span_list: true,
			with_ansi: true,
		}
	}

	/// Create a preset configuration for production
	pub fn production() -> Self {
		Self {
			log_level: "info".to_string(),
			format: LogFormat::Json,
			output: LogOutput::Both {
				console: Box::new(LogOutput::Stderr),
				file: PathBuf::from("logs/gmn.log"),
			},
			with_timestamps: true,
			with_thread_ids: true,
			with_thread_names: true,
			with_file_line: false,
			with_span_list: false,
			with_ansi: false,
		}
	}

	/// Create a preset configuration for testing
	pub fn testing() -> Self {
		Self {
			log_level: "warn".to_string(),
			format: LogFormat::Compact,
			output: LogOutput::Stderr,
			with_timestamps: false,
			with_thread_ids: false,
			with_thread_names: false,
			with_file_line: false,
			with_span_list: false,
			with_ansi: false,
		}
	}

	/// Builder method to set log level
	pub fn with_log_level(mut self, level: impl Into<String>) -> Self {
		self.log_level = level.into();
		self
	}

	/// Builder method to set format
	pub fn with_format(mut self, format: LogFormat) -> Self {
		self.format = format;
		self
	}

	/// Builder method to set output
	pub fn with_output(mut self, output: LogOutput) -> Self {
		self.output = output;
		self
	}

	/// Builder method to enable/disable timestamps
	pub fn with_timestamps(mut self, enabled: bool) -> Self {
		self.with_timestamps = enabled;
		self
	}

	/// Builder method to enable/disable thread IDs
	pub fn with_thread_ids(mut self, enabled: bool) -> Self {
		self.with_thread_ids = enabled;
		self
	}

	/// Builder method to enable/disable thread names
	pub fn with_thread_names(mut self, enabled: bool) -> Self {
		self.with_thread_names = enabled;
		self
	}

	/// Builder method to enable/disable file/line information
	pub fn with_file_line(mut self, enabled: bool) -> Self {
		self.with_file_line = enabled;
		self
	}

	/// Builder method to enable/disable span list
	pub fn with_span_list(mut self, enabled: bool) -> Self {
		self.with_span_list = enabled;
		self
	}

	/// Builder method to enable/disable ANSI colors
	pub fn with_ansi(mut self, enabled: bool) -> Self {
		self.with_ansi = enabled;
		self
	}
}
