//! Error types for the GabMacN core library.
//!
//! This module defines a comprehensive error hierarchy using `thiserror` for
//! type-safe error handling throughout the gmn-core ecosystem.

use std::fmt;

/// Shared metadata contract for domain errors in `gmn_core`.
///
/// This trait exists to reduce boilerplate when exposing code/hint/context from
/// top-level wrapper errors like [`GmnError`].
pub trait ErrorMetadata {
	/// Stable error code for searchability and diagnostics.
	fn code(&self) -> &'static str;

	/// Optional remediation hint for users/operators.
	fn hint(&self) -> Option<&str> {
		None
	}

	/// Optional additional context details.
	fn context(&self) -> Option<String> {
		None
	}
}

/// Result type alias for gmn-core operations
pub type Result<T> = std::result::Result<T, GmnError>;

/// Main error type for the GabMacN core library
#[derive(Debug, thiserror::Error)]
pub enum GmnError {
	/// Generic reusable application errors
	#[error("Generic error: {0}")]
	Generic(#[from] GenericError),

	/// Configuration-related errors
	#[error("Configuration error: {0}")]
	Config(#[from] ConfigError),

	/// Tracing/logging initialization errors
	#[error("Tracing error: {0}")]
	Tracing(#[from] TracingError),

	/// CLI errors
	#[error("CLI error: {0}")]
	CLI(#[from] CLIError),

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
			Self::Internal(_) => "GMN-000",
			other => other.metadata().code(),
		}
	}

	/// Get a hint for resolving this error, if available
	pub fn hint(&self) -> Option<&str> {
		match self {
			Self::Internal(_) => None,
			other => other.metadata().hint(),
		}
	}

	/// Get additional context for this error, if available
	pub fn context(&self) -> Option<String> {
		match self {
			Self::Internal(_) => None,
			other => other.metadata().context(),
		}
	}

	fn metadata(&self) -> &dyn ErrorMetadata {
		match self {
			Self::Generic(e) => e,
			Self::Config(e) => e,
			Self::Tracing(e) => e,
			Self::CLI(e) => e,
			Self::Database(e) => e,
			Self::Auth(e) => e,
			Self::RateLimit(e) => e,
			Self::Api(e) => e,
			Self::Internal(_) => unreachable!("internal is handled separately"),
		}
	}
}

// ============================================================================
// Generic Errors
// ============================================================================

/// Reusable generic errors for consumers who don't need a dedicated domain type.
#[derive(Debug, thiserror::Error)]
pub enum GenericError {
	/// Requested entity was not found.
	#[error("Not found: {entity}")]
	NotFound {
		/// Entity or resource name.
		entity: String,
		/// Optional additional details.
		message: Option<String>,
	},

	/// Caller is not authenticated.
	#[error("Unauthorized: {message}")]
	Unauthorized {
		/// Human-readable reason.
		message: String,
	},

	/// Caller is authenticated but lacks required permissions.
	#[error("Forbidden: {message}")]
	Forbidden {
		/// Human-readable reason.
		message: String,
	},

	/// Input failed validation.
	#[error("Validation failed: {field}")]
	Validation {
		/// Field or logical component.
		field: String,
		/// Validation message.
		message: String,
	},

	/// Operation conflicts with current resource state.
	#[error("Conflict: {resource}")]
	Conflict {
		/// Resource name.
		resource: String,
		/// Optional details.
		message: Option<String>,
	},

	/// Downstream or local operation timed out.
	#[error("Timeout while performing operation: {operation}")]
	Timeout {
		/// Operation name.
		operation: String,
		/// Optional timeout threshold in milliseconds.
		timeout_ms: Option<u64>,
	},

	/// Request exceeds acceptable limits.
	#[error("Payload too large")]
	PayloadTooLarge {
		/// Optional limit in bytes.
		limit_bytes: Option<u64>,
	},

	/// Request is not processable due to semantic/business constraints.
	#[error("Unprocessable entity: {message}")]
	UnprocessableEntity {
		/// Human-readable reason.
		message: String,
	},

	/// Unsupported operation/type/feature.
	#[error("Unsupported operation: {operation}")]
	Unsupported {
		/// Unsupported operation name.
		operation: String,
		/// Optional details.
		message: Option<String>,
	},

	/// Service or dependency is unavailable.
	#[error("Service unavailable: {service}")]
	ServiceUnavailable {
		/// Service/dependency name.
		service: String,
		/// Optional details.
		message: Option<String>,
	},

	/// Catch-all generic error.
	#[error("{message}")]
	Other {
		/// Human-readable message.
		message: String,
	},
}

impl ErrorMetadata for GenericError {
	fn code(&self) -> &'static str {
		match self {
			Self::NotFound { .. } => "GMN-GEN-001",
			Self::Unauthorized { .. } => "GMN-GEN-002",
			Self::Forbidden { .. } => "GMN-GEN-003",
			Self::Validation { .. } => "GMN-GEN-004",
			Self::Conflict { .. } => "GMN-GEN-005",
			Self::Timeout { .. } => "GMN-GEN-006",
			Self::PayloadTooLarge { .. } => "GMN-GEN-007",
			Self::UnprocessableEntity { .. } => "GMN-GEN-008",
			Self::Unsupported { .. } => "GMN-GEN-009",
			Self::ServiceUnavailable { .. } => "GMN-GEN-010",
			Self::Other { .. } => "GMN-GEN-011",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::NotFound { .. } => {
				Some("Check whether the resource exists and verify the identifier/path")
			}
			Self::Unauthorized { .. } => {
				Some("Authenticate and provide valid credentials before retrying")
			}
			Self::Forbidden { .. } => {
				Some("Ensure your account/token has the required permissions")
			}
			Self::Validation { .. } => Some("Fix the invalid field values and retry the request"),
			Self::Conflict { .. } => {
				Some("Refresh state and retry, or resolve the conflicting resource changes")
			}
			Self::Timeout { .. } => Some(
				"Retry with backoff or increase timeout if the operation is expected to take longer",
			),
			Self::PayloadTooLarge { .. } => {
				Some("Reduce payload size, split the request, or increase server-side limits")
			}
			Self::UnprocessableEntity { .. } => {
				Some("Adjust request semantics to satisfy business rules")
			}
			Self::Unsupported { .. } => {
				Some("Use a supported operation/format or upgrade to a compatible version")
			}
			Self::ServiceUnavailable { .. } => {
				Some("Retry later; dependency may be degraded or under maintenance")
			}
			Self::Other { .. } => None,
		}
	}

	fn context(&self) -> Option<String> {
		match self {
			Self::NotFound { entity, message } => {
				let mut ctx = format!("entity={}", entity);
				if let Some(msg) = message {
					ctx.push_str(&format!(", details={}", msg));
				}
				Some(ctx)
			}
			Self::Unauthorized { message } => Some(format!("details={}", message)),
			Self::Forbidden { message } => Some(format!("details={}", message)),
			Self::Validation { field, message } => {
				Some(format!("field={}, details={}", field, message))
			}
			Self::Conflict { resource, message } => {
				let mut ctx = format!("resource={}", resource);
				if let Some(msg) = message {
					ctx.push_str(&format!(", details={}", msg));
				}
				Some(ctx)
			}
			Self::Timeout { operation, timeout_ms } => {
				let mut ctx = format!("operation={}", operation);
				if let Some(ms) = timeout_ms {
					ctx.push_str(&format!(", timeout_ms={}", ms));
				}
				Some(ctx)
			}
			Self::PayloadTooLarge { limit_bytes } => {
				limit_bytes.map(|b| format!("limit_bytes={}", b))
			}
			Self::UnprocessableEntity { message } => Some(format!("details={}", message)),
			Self::Unsupported { operation, message } => {
				let mut ctx = format!("operation={}", operation);
				if let Some(msg) = message {
					ctx.push_str(&format!(", details={}", msg));
				}
				Some(ctx)
			}
			Self::ServiceUnavailable { service, message } => {
				let mut ctx = format!("service={}", service);
				if let Some(msg) = message {
					ctx.push_str(&format!(", details={}", msg));
				}
				Some(ctx)
			}
			Self::Other { message } => Some(format!("details={}", message)),
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

impl ErrorMetadata for ConfigError {
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
			Self::InvalidLogLevel { .. } => Some(
				"Valid log levels: trace, debug, info, warn, error. You can also use directive syntax like 'gmn_core=debug,hyper=info'",
			),
			Self::InvalidLogFormat { .. } => Some("Valid formats: pretty, compact, json"),
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

impl ErrorMetadata for TracingError {
	fn code(&self) -> &'static str {
		match self {
			Self::AlreadyInitialized => "GMN-TRC-001",
			Self::FileCreationFailed { .. } => "GMN-TRC-002",
			Self::SetGlobalFailed { .. } => "GMN-TRC-003",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::AlreadyInitialized => Some(
				"Tracing can only be initialized once. Call init_tracing() early in your application startup.",
			),
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
			Self::FileCreationFailed { path, .. } => Some(format!("Log file path: {}", path)),
			_ => None,
		}
	}
}

// ============================================================================
// CLI Errors
// ============================================================================

/// CLI-related errors
#[derive(Debug, thiserror::Error)]
pub enum CLIError {
	/// Fetch error
	#[error("Failed to fetch CLI input: {message}")]
	FetchError {
		/// The underlying error message
		message: String,
	},

	/// Operation cancelled by user
	#[error("Operation cancelled by user")]
	Cancelled {
		/// The underlying error message
		message: Option<String>,
	},
}

impl ErrorMetadata for CLIError {
	fn code(&self) -> &'static str {
		match self {
			Self::FetchError { .. } => "GMN-CLI-001",
			Self::Cancelled { .. } => "GMN-CLI-002",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::FetchError { .. } => {
				Some("This error occurs when the prompt fails to read input")
			}
			Self::Cancelled { .. } => {
				Some("The user cancelled the operation, no further action needed")
			}
		}
	}

	fn context(&self) -> Option<String> {
		match self {
			Self::FetchError { message } => Some(format!("Underlying error: {}", message)),
			Self::Cancelled { message } => {
				message.as_ref().map(|m| format!("Cancellation message: {}", m))
			}
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

impl ErrorMetadata for DatabaseError {
	fn code(&self) -> &'static str {
		match self {
			Self::ConnectionFailed(_) => "GMN-DB-001",
			Self::QueryFailed(_) => "GMN-DB-002",
			Self::TransactionFailed(_) => "GMN-DB-003",
		}
	}

	fn hint(&self) -> Option<&str> {
		match self {
			Self::ConnectionFailed(_) => {
				Some("Check database connection string and network connectivity")
			}
			Self::QueryFailed(_) => Some("Verify query syntax and database schema"),
			Self::TransactionFailed(_) => Some("Check for conflicts or constraint violations"),
		}
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

impl ErrorMetadata for AuthError {
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

impl ErrorMetadata for RateLimitError {
	fn code(&self) -> &'static str {
		"GMN-RATE-001"
	}

	fn hint(&self) -> Option<&str> {
		Some("Wait before making additional requests or upgrade your plan for higher limits")
	}

	fn context(&self) -> Option<String> {
		Some(format!("Requests: {}/{} in {} seconds", self.requests, self.limit, self.window_secs))
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
	ServerError {
		/// HTTP status code
		status: u16,
		/// Error message from server
		message: String,
	},
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

impl ErrorMetadata for ApiError {
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
