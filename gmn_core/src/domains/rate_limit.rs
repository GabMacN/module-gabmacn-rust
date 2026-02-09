//! Rate limiting tracing utilities.
//!
//! This module provides tracing helpers for rate limiting operations.

use tracing::Span;

/// Rate limit check result
#[derive(Debug, Clone, Copy)]
pub enum RateLimitResult {
	/// Request allowed
	Allowed,
	/// Request denied (rate limit exceeded)
	Denied,
	/// Rate limit check skipped
	Skipped,
}

impl RateLimitResult {
	/// Get the string representation of the result
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Allowed => "allowed",
			Self::Denied => "denied",
			Self::Skipped => "skipped",
		}
	}
}

/// Create a span for a rate limit check
pub fn rate_limit_check_span(resource: &str, identifier: &str) -> Span {
	tracing::info_span!(
		"rate_limit_check",
		resource = resource,
		identifier = identifier,
		result = tracing::field::Empty,
		requests = tracing::field::Empty,
		limit = tracing::field::Empty,
		window_secs = tracing::field::Empty,
		remaining = tracing::field::Empty,
	)
}

/// Create a span for rate limit configuration
pub fn rate_limit_config_span(resource: &str) -> Span {
	tracing::info_span!(
		"rate_limit_config",
		resource = resource,
		limit = tracing::field::Empty,
		window_secs = tracing::field::Empty,
	)
}

/// Record rate limit check result
pub fn record_check_result(
	span: &Span,
	result: RateLimitResult,
	requests: u32,
	limit: u32,
	window_secs: u64,
	remaining: u32,
) {
	span.record("result", result.as_str());
	span.record("requests", requests);
	span.record("limit", limit);
	span.record("window_secs", window_secs);
	span.record("remaining", remaining);
}

/// Record rate limit configuration
pub fn record_config(span: &Span, limit: u32, window_secs: u64) {
	span.record("limit", limit);
	span.record("window_secs", window_secs);
}

/// Log a rate limit violation event
pub fn log_rate_limit_exceeded(resource: &str, identifier: &str, requests: u32, limit: u32) {
	tracing::warn!(
		resource = resource,
		identifier = identifier,
		requests = requests,
		limit = limit,
		"Rate limit exceeded"
	);
}
