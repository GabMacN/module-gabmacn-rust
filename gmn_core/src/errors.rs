//! Error types for the GabMacN core library.
//!
//! This module defines a comprehensive error hierarchy using `thiserror` for
//! type-safe error handling throughout the gmn-core ecosystem.

use std::fmt;

/// Result type alias for gmn-core operations
pub type Result<T> = std::result::Result<T, GmnError>;

/// Main error type for the GabMacN core library
#[derive(Debug, thiserror::Error)]
pub enum GmnError {
	/// Configuration-related errors
	#[error("Configuration error: {0}")]
	Config(#[from] ConfigError),

	/// Tracing/logging initialization errors
	#[error("Tracing error: {0}")]
	Tracing(#[from] TracingError),

	/// Database-related errors (placeholder for future implementation)
	#[error("Database error: {0}")]
	Database(#[from] DatabaseError),

	/// Authentication/authorization errors (placeholder for future implementation)
	#[error("Authentication error: {0}")]
	Auth(#[from] AuthError),

	/// Rate limiting errors (placeholder for future implementation)
	#[error("Rate limit exceeded: {0}")]
	RateLimit(#[from] RateLimitError),

	/// API/network errors (placeholder for future implementation)
	#[error("API error: {0}")]
	Api(#[from] ApiError),

	/// Generic internal error
	#[error("Internal error: {0}")]
	Internal(String),
}

impl GmnError {
	/// Get the error code for this error
	pub fn code(&self) -> &'static str {
		match self {
			Self::Config(e) => e.code(),
			Self::Tracing(e) => e.code(),
			Self::Database(e) => e.code(),
			Self::Auth(e) => e.code(),
			Self::RateLimit(e) => e.code(),
			Self::Api(e) => e.code(),
			Self::Internal(_) => "GMN-000",
		}
	}

	/// Get a hint for resolving this error, if available
	pub fn hint(&self) -> Option<&str> {
		match self {
			Self::Config(e) => e.hint(),
			Self::Tracing(e) => e.hint(),
			Self::Database(e) => e.hint(),
			Self::Auth(e) => e.hint(),
			Self::RateLimit(e) => e.hint(),
			Self::Api(e) => e.hint(),
			Self::Internal(_) => None,
		}
	}

	/// Get additional context for this error, if available
	pub fn context(&self) -> Option<String> {
		match self {
			Self::Config(e) => e.context(),
			Self::Tracing(e) => e.context(),
			Self::Database(e) => e.context(),
			Self::Auth(e) => e.context(),
			Self::RateLimit(e) => e.context(),
			Self::Api(e) => e.context(),
			Self::Internal(_) => None,
		}
	}
}

// ============================================================================
// Configuration Errors
// ============================================================================

/// Configuration-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	/// Invalid log level specified
	#[error("Invalid log level: {level}")]
	InvalidLogLevel {
		/// The invalid log level string
		level: String,
	},

	/// Invalid log format specified
	#[error("Invalid log format: {format}")]
	InvalidLogFormat {
		/// The invalid format string
		format: String,
	},

	/// Invalid output path
	#[error("Invalid output path: {path}")]
	InvalidOutputPath {
		/// The invalid path
		path: String,
		/// The underlying error
		#[source]
		source: std::io::Error,
	},

	/// Environment variable parsing error
	#[error("Failed to parse environment variable {var}: {value}")]
	EnvVarParse {
		/// The environment variable name
		var: String,
		/// The value that failed to parse
		value: String,
	},
}

impl ConfigError {
	fn code(&self) -> &'static str {
		match self {
			Self::InvalidLogLevel { .. } => "GMN-CFG-001",
			Self::InvalidLogFormat { .. } => "GMN-CFG-002",
			Self::InvalidOutputPath { .. } => "GMN-CFG-003",
			Self::EnvVarParse { .. } => "GMN-CFG-004",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::InvalidLogLevel { .. } => {
				Some("Valid log levels: trace, debug, info, warn, error. You can also use directive syntax like 'gmn_core=debug,hyper=info'")
			}
			Self::InvalidLogFormat { .. } => {
				Some("Valid formats: pretty, compact, json")
			}
			Self::InvalidOutputPath { .. } => {
				Some("Ensure the directory exists and you have write permissions")
			}
			Self::EnvVarParse { .. } => {
				Some("Check the environment variable value matches the expected format")
			}
		}
	}

	fn context(&self) -> Option<String> {
		match self {
			Self::InvalidOutputPath { path, .. } => {
				Some(format!("Attempted to use path: {}", path))
			}
			Self::EnvVarParse { var, value } => {
				Some(format!("Variable: {}, Value: {}", var, value))
			}
			_ => None,
		}
	}
}

// ============================================================================
// Tracing Errors
// ============================================================================

/// Tracing/logging initialization errors
#[derive(Debug, thiserror::Error)]
pub enum TracingError {
	/// Tracing subscriber already initialized
	#[error("Tracing subscriber already initialized")]
	AlreadyInitialized,

	/// Failed to create log file
	#[error("Failed to create log file: {path}")]
	FileCreationFailed {
		/// The file path
		path: String,
		/// The underlying error
		#[source]
		source: std::io::Error,
	},

	/// Failed to set global subscriber
	#[error("Failed to set global tracing subscriber")]
	SetGlobalFailed {
		/// The underlying error
		#[source]
		source: tracing::subscriber::SetGlobalDefaultError,
	},
}

impl TracingError {
	fn code(&self) -> &'static str {
		match self {
			Self::AlreadyInitialized => "GMN-TRC-001",
			Self::FileCreationFailed { .. } => "GMN-TRC-002",
			Self::SetGlobalFailed { .. } => "GMN-TRC-003",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::AlreadyInitialized => {
				Some("Tracing can only be initialized once. Call init_tracing() early in your application startup.")
			}
			Self::FileCreationFailed { .. } => {
				Some("Ensure the log directory exists and you have write permissions")
			}
			Self::SetGlobalFailed { .. } => {
				Some("This usually indicates tracing was already initialized elsewhere")
			}
		}
	}

	fn context(&self) -> Option<String> {
		match self {
			Self::FileCreationFailed { path, .. } => {
				Some(format!("Log file path: {}", path))
			}
			_ => None,
		}
	}
}

// ============================================================================
// Database Errors (Placeholder)
// ============================================================================

/// Database-related errors (placeholder for future implementation)
#[derive(Debug)]
pub enum DatabaseError {
	/// Connection failed
	ConnectionFailed(String),
	/// Query failed
	QueryFailed(String),
	/// Transaction failed
	TransactionFailed(String),
}

impl fmt::Display for DatabaseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
			Self::QueryFailed(msg) => write!(f, "Database query failed: {}", msg),
			Self::TransactionFailed(msg) => write!(f, "Database transaction failed: {}", msg),
		}
	}
}

impl std::error::Error for DatabaseError {}

impl DatabaseError {
	fn code(&self) -> &'static str {
		match self {
			Self::ConnectionFailed(_) => "GMN-DB-001",
			Self::QueryFailed(_) => "GMN-DB-002",
			Self::TransactionFailed(_) => "GMN-DB-003",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::ConnectionFailed(_) => Some("Check database connection string and network connectivity"),
			Self::QueryFailed(_) => Some("Verify query syntax and database schema"),
			Self::TransactionFailed(_) => Some("Check for conflicts or constraint violations"),
		}
	}

	fn context(&self) -> Option<String> {
		None
	}
}

// ============================================================================
// Auth Errors (Placeholder)
// ============================================================================

/// Authentication/authorization errors (placeholder for future implementation)
#[derive(Debug)]
pub enum AuthError {
	/// Invalid credentials
	InvalidCredentials,
	/// Token expired
	TokenExpired,
	/// Insufficient permissions
	InsufficientPermissions,
}

impl fmt::Display for AuthError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidCredentials => write!(f, "Invalid credentials"),
			Self::TokenExpired => write!(f, "Authentication token expired"),
			Self::InsufficientPermissions => write!(f, "Insufficient permissions"),
		}
	}
}

impl std::error::Error for AuthError {}

impl AuthError {
	fn code(&self) -> &'static str {
		match self {
			Self::InvalidCredentials => "GMN-AUTH-001",
			Self::TokenExpired => "GMN-AUTH-002",
			Self::InsufficientPermissions => "GMN-AUTH-003",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::InvalidCredentials => Some("Verify your API key or credentials"),
			Self::TokenExpired => Some("Refresh your authentication token"),
			Self::InsufficientPermissions => Some("Contact administrator for required permissions"),
		}
	}

	fn context(&self) -> Option<String> {
		None
	}
}

// ============================================================================
// Rate Limit Errors (Placeholder)
// ============================================================================

/// Rate limiting errors (placeholder for future implementation)
#[derive(Debug)]
pub struct RateLimitError {
	/// Number of requests made
	pub requests: u32,
	/// Maximum allowed requests
	pub limit: u32,
	/// Time window in seconds
	pub window_secs: u64,
}

impl fmt::Display for RateLimitError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Rate limit exceeded: {} requests in {} seconds (limit: {})",
			self.requests, self.window_secs, self.limit
		)
	}
}

impl std::error::Error for RateLimitError {}

impl RateLimitError {
	fn code(&self) -> &'static str {
		"GMN-RATE-001"
	}

	fn hint(&self) -> Option<&str> {
		Some("Wait before making additional requests or upgrade your plan for higher limits")
	}

	fn context(&self) -> Option<String> {
		Some(format!(
			"Requests: {}/{} in {} seconds",
			self.requests, self.limit, self.window_secs
		))
	}
}

// ============================================================================
// API Errors (Placeholder)
// ============================================================================

/// API/network errors (placeholder for future implementation)
#[derive(Debug)]
pub enum ApiError {
	/// Network request failed
	NetworkError(String),
	/// Invalid response
	InvalidResponse(String),
	/// Server error
	ServerError { status: u16, message: String },
}

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
			Self::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
			Self::ServerError { status, message } => {
				write!(f, "Server error ({}): {}", status, message)
			}
		}
	}
}

impl std::error::Error for ApiError {}

impl ApiError {
	fn code(&self) -> &'static str {
		match self {
			Self::NetworkError(_) => "GMN-API-001",
			Self::InvalidResponse(_) => "GMN-API-002",
			Self::ServerError { .. } => "GMN-API-003",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::NetworkError(_) => Some("Check network connectivity and firewall settings"),
			Self::InvalidResponse(_) => Some("The API response format may have changed"),
			Self::ServerError { status, .. } if *status >= 500 => {
				Some("Server is experiencing issues, try again later")
			}
			Self::ServerError { .. } => Some("Check request parameters and authentication"),
		}
	}

	fn context(&self) -> Option<String> {
		match self {
			Self::ServerError { status, .. } => Some(format!("HTTP status: {}", status)),
			_ => None,
		}
	}
}
