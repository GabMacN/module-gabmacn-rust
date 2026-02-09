//! Authentication and session tracing utilities.
//!
//! This module provides tracing helpers for authentication and session management.

use tracing::Span;

/// Authentication operation types
#[derive(Debug, Clone, Copy)]
pub enum AuthOperation {
	/// User login
	Login,
	/// User logout
	Logout,
	/// Token validation
	ValidateToken,
	/// Token refresh
	RefreshToken,
	/// Session creation
	CreateSession,
	/// Session validation
	ValidateSession,
	/// Session termination
	TerminateSession,
	/// API key validation
	ValidateApiKey,
}

impl AuthOperation {
	/// Get the string representation of the operation
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Login => "login",
			Self::Logout => "logout",
			Self::ValidateToken => "validate_token",
			Self::RefreshToken => "refresh_token",
			Self::CreateSession => "create_session",
			Self::ValidateSession => "validate_session",
			Self::TerminateSession => "terminate_session",
			Self::ValidateApiKey => "validate_api_key",
		}
	}
}

/// Create a span for an authentication operation
pub fn auth_span(operation: AuthOperation, user_id: Option<&str>) -> Span {
	if let Some(uid) = user_id {
		tracing::info_span!(
			"auth_operation",
			operation = operation.as_str(),
			user_id = uid,
			success = tracing::field::Empty,
			duration_ms = tracing::field::Empty,
		)
	} else {
		tracing::info_span!(
			"auth_operation",
			operation = operation.as_str(),
			success = tracing::field::Empty,
			duration_ms = tracing::field::Empty,
		)
	}
}

/// Create a span for session operations
pub fn session_span(operation: AuthOperation, session_id: &str) -> Span {
	tracing::info_span!(
		"session_operation",
		operation = operation.as_str(),
		session_id = session_id,
		success = tracing::field::Empty,
		duration_ms = tracing::field::Empty,
	)
}

/// Create a span for API key validation
pub fn api_key_span(key_prefix: &str) -> Span {
	tracing::info_span!(
		"api_key_validation",
		key_prefix = key_prefix,
		valid = tracing::field::Empty,
		duration_ms = tracing::field::Empty,
	)
}

/// Record authentication result
pub fn record_auth_result(span: &Span, success: bool, duration_ms: u64) {
	span.record("success", success);
	span.record("duration_ms", duration_ms);
}

/// Record API key validation result
pub fn record_api_key_result(span: &Span, valid: bool, duration_ms: u64) {
	span.record("valid", valid);
	span.record("duration_ms", duration_ms);
}
