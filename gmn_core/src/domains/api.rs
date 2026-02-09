//! API request/response tracing utilities.
//!
//! This module provides tracing helpers for API operations.

use tracing::Span;

/// HTTP methods
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
	/// GET request
	Get,
	/// POST request
	Post,
	/// PUT request
	Put,
	/// PATCH request
	Patch,
	/// DELETE request
	Delete,
	/// HEAD request
	Head,
	/// OPTIONS request
	Options,
}

impl HttpMethod {
	/// Get the string representation of the HTTP method
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Get => "GET",
			Self::Post => "POST",
			Self::Put => "PUT",
			Self::Patch => "PATCH",
			Self::Delete => "DELETE",
			Self::Head => "HEAD",
			Self::Options => "OPTIONS",
		}
	}
}

/// Create a span for an API request
pub fn request_span(method: HttpMethod, path: &str) -> Span {
	tracing::info_span!(
		"api_request",
		method = method.as_str(),
		path = path,
		status_code = tracing::field::Empty,
		duration_ms = tracing::field::Empty,
		request_id = tracing::field::Empty,
	)
}

/// Create a span for an API request with request ID
pub fn request_span_with_id(method: HttpMethod, path: &str, request_id: &str) -> Span {
	tracing::info_span!(
		"api_request",
		method = method.as_str(),
		path = path,
		request_id = request_id,
		status_code = tracing::field::Empty,
		duration_ms = tracing::field::Empty,
	)
}

/// Create a span for an external API call
pub fn external_api_span(service: &str, endpoint: &str) -> Span {
	tracing::info_span!(
		"external_api_call",
		service = service,
		endpoint = endpoint,
		status_code = tracing::field::Empty,
		duration_ms = tracing::field::Empty,
	)
}

/// Record API request completion
pub fn record_request_completion(span: &Span, status_code: u16, duration_ms: u64) {
	span.record("status_code", status_code);
	span.record("duration_ms", duration_ms);
}

/// Record request ID
pub fn record_request_id(span: &Span, request_id: &str) {
	span.record("request_id", request_id);
}
