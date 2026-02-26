//! # A showcase of integrating third-party errors with gmn's display metadata model.
//!
//! This example simulates a third-party SDK error type and demonstrates how to create a local adapter that implements `DisplayMetadata` for rich integration with gmn's display utilities. It also shows a fallback path for displaying errors without a custom adapter.

use gmn_core::error_display::{
	DisplayMessage, DisplayMetadata, display_error, display_std_error, display_success,
};
use gmn_core::tracing::init_tracing;
use std::error::Error;
use std::fmt;

/// Simulated third-party SDK error (owned by another crate).
#[derive(Debug)]
pub enum AcmeSdkError {
	/// Represents a failed HTTP request to the ACME API.
	RequestFailed {
		/// The API endpoint that was called.
		endpoint: String,
		/// The HTTP status code returned by the API.
		status: u16,
	},
	/// Represents a rate limit response from the ACME API, with a retry-after duration.
	RateLimited {
		/// The number of seconds after which the request can be retried.
		retry_after_secs: u64,
	},
	/// Represents an authentication error due to an invalid or missing API key.
	InvalidApiKey,
}

impl fmt::Display for AcmeSdkError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::RequestFailed { endpoint, status } => {
				write!(f, "Request to '{}' failed with status {}", endpoint, status)
			}
			Self::RateLimited { retry_after_secs } => {
				write!(f, "Rate limited by provider, retry after {}s", retry_after_secs)
			}
			Self::InvalidApiKey => write!(f, "API key is invalid or missing"),
		}
	}
}

impl Error for AcmeSdkError {}

/// Local adapter that maps third-party errors to the gmn display metadata model.
///
/// In real code, this adapter would typically live in your integration layer.
struct AcmeSdkErrorDisplay<'a> {
	error: &'a AcmeSdkError,
}

impl<'a> DisplayMetadata for AcmeSdkErrorDisplay<'a> {
	fn title(&self) -> &str {
		match self.error {
			AcmeSdkError::RequestFailed { .. } => "ACME SDK Request Error",
			AcmeSdkError::RateLimited { .. } => "ACME SDK Rate Limited",
			AcmeSdkError::InvalidApiKey => "ACME SDK Authentication Error",
		}
	}

	fn code(&self) -> &str {
		match self.error {
			AcmeSdkError::RequestFailed { .. } => "ACME-SDK-HTTP-001",
			AcmeSdkError::RateLimited { .. } => "ACME-SDK-RATE-001",
			AcmeSdkError::InvalidApiKey => "ACME-SDK-AUTH-001",
		}
	}

	fn message(&self) -> String {
		self.error.to_string()
	}

	fn context(&self) -> Option<String> {
		match self.error {
			AcmeSdkError::RequestFailed { endpoint, status } => {
				Some(format!("endpoint='{}', status={}", endpoint, status))
			}
			AcmeSdkError::RateLimited { retry_after_secs } => {
				Some(format!("retry_after_secs={}", retry_after_secs))
			}
			AcmeSdkError::InvalidApiKey => Some("API key loaded from env var ACME_API_KEY".into()),
		}
	}

	fn hint(&self) -> Option<&str> {
		match self.error {
			AcmeSdkError::RequestFailed { .. } => {
				Some("Check provider status and inspect request payload/headers.")
			}
			AcmeSdkError::RateLimited { .. } => Some("Back off and retry with exponential delay."),
			AcmeSdkError::InvalidApiKey => {
				Some("Set a valid ACME_API_KEY and restart the application.")
			}
		}
	}
}

fn third_party_operation() -> Result<(), AcmeSdkError> {
	Err(AcmeSdkError::RequestFailed { endpoint: "/v1/customers/42".to_string(), status: 503 })
}

fn third_party_operation_fallback() -> Result<(), AcmeSdkError> {
	Err(AcmeSdkError::RateLimited { retry_after_secs: 30 })
}

fn main() {
	let _ = init_tracing();

	if let Err(err) = third_party_operation() {
		// Preferred path: rich integration via DisplayMetadata adapter.
		let adapter = AcmeSdkErrorDisplay { error: &err };
		display_error(&adapter);
	}

	if let Err(err) = third_party_operation_fallback() {
		// Fallback path: display any std::error::Error without custom adapter.
		display_std_error(
			&err,
			Some("Third-Party Integration Error"),
			Some("INTEGRATION-001"),
			Some("Occurred while calling ACME SDK from billing sync"),
			Some("Retry later or degrade gracefully."),
		);
	}

	display_success(&DisplayMessage {
		title: "Example Complete",
		code: "EXAMPLE-OK",
		message: "Third-party error display integration demo finished.",
		context: None,
		hint: None,
	});
}
