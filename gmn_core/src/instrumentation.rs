//! Instrumentation utilities for common tracing patterns.
//!
//! This module provides helper macros and functions to simplify instrumentation
//! of operations, performance measurement, and event logging.

/// Create a span for an operation with automatic field capture
///
/// # Example
///
/// ```no_run
/// use gmn_core::instrumentation::trace_operation;
///
/// fn process_user(user_id: u64) {
///     let _span = trace_operation!("process_user", user_id);
///     // Your code here
/// }
/// ```
#[macro_export]
macro_rules! trace_operation {
	($name:expr) => {
		tracing::info_span!($name)
	};
	($name:expr, $($field:tt)*) => {
		tracing::info_span!($name, $($field)*)
	};
}

/// Measure the duration of an operation and log it
///
/// # Example
///
/// ```no_run
/// use gmn_core::instrumentation::measure_duration;
///
/// fn expensive_operation() {
///     measure_duration!("expensive_operation", {
///         // Your expensive code here
///         std::thread::sleep(std::time::Duration::from_millis(100));
///     });
/// }
/// ```
#[macro_export]
macro_rules! measure_duration {
	($name:expr, $body:block) => {{
		let start = std::time::Instant::now();
		let result = $body;
		let duration = start.elapsed();
		tracing::info!(
			operation = $name,
			duration_ms = duration.as_millis(),
			"Operation completed"
		);
		result
	}};
}

/// Log an event with context
///
/// # Example
///
/// ```no_run
/// use gmn_core::instrumentation::log_event;
///
/// fn handle_request() {
///     log_event!(info, "request_received", request_id = "abc123", method = "GET");
/// }
/// ```
#[macro_export]
macro_rules! log_event {
	($level:ident, $message:expr) => {
		tracing::$level!($message)
	};
	($level:ident, $message:expr, $($field:tt)*) => {
		tracing::$level!($($field)*, $message)
	};
}

/// Helper to create a span with common database operation fields
pub fn db_operation_span(operation: &str, table: &str) -> tracing::Span {
	tracing::info_span!("db_operation", operation = operation, table = table)
}

/// Helper to create a span with common API request fields
pub fn api_request_span(method: &str, path: &str) -> tracing::Span {
	tracing::info_span!("api_request", method = method, path = path)
}

/// Helper to create a span for authentication operations
pub fn auth_operation_span(operation: &str, user_id: Option<&str>) -> tracing::Span {
	if let Some(uid) = user_id {
		tracing::info_span!("auth_operation", operation = operation, user_id = uid)
	} else {
		tracing::info_span!("auth_operation", operation = operation)
	}
}

/// Helper to create a span for rate limiting checks
pub fn rate_limit_span(resource: &str, identifier: &str) -> tracing::Span {
	tracing::info_span!("rate_limit_check", resource = resource, identifier = identifier)
}

/// Record an error in the current span
pub fn record_error(error: &dyn std::error::Error) {
	tracing::error!(error = %error, "Error occurred");
}

/// Record an error with additional context
pub fn record_error_with_context(error: &dyn std::error::Error, context: &str) {
	tracing::error!(error = %error, context = context, "Error occurred");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_span_creation() {
		let span = db_operation_span("SELECT", "users");
		assert_eq!(span.metadata().unwrap().name(), "db_operation");
	}

	#[test]
	fn test_api_span_creation() {
		let span = api_request_span("GET", "/api/users");
		assert_eq!(span.metadata().unwrap().name(), "api_request");
	}
}
