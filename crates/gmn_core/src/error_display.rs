//! Error display integration with the pretty error printing module.
//!
//! This module bridges structured metadata + message content to the terminal
//! renderer provided by the `print_pretty_error` module.
//!
//! All display functions automatically emit corresponding tracing events so that
//! user-visible output is also captured in logs for observability.

use crate::errors::GmnError;
use crate::print_pretty_error::{PrettyMessageLevel, print_pretty_message};

/// Shared metadata for displayable diagnostics/messages.
///
/// Implement this on your own error or status types to integrate with
/// `display_error_from`, `display_warning_from`, `display_info_from`,
/// and `display_success_from` without requiring a `GmnError`.
pub trait DisplayMetadata {
	/// Human-readable title shown in the message header.
	fn title(&self) -> &str;

	/// Stable code identifier (e.g. `GMN-CFG-001`).
	fn code(&self) -> &str;

	/// Primary user-facing message body.
	fn message(&self) -> String;

	/// Optional contextual detail.
	fn context(&self) -> Option<String> {
		None
	}

	/// Optional actionable hint.
	fn hint(&self) -> Option<&str> {
		None
	}
}

/// Lightweight generic display payload for warnings/info/success and ad-hoc errors.
#[derive(Debug, Clone)]
pub struct DisplayMessage<'a> {
	/// Human-readable title shown in the message header.
	pub title: &'a str,
	/// Stable code identifier (e.g. `GMN-WARN-001`).
	pub code: &'a str,
	/// Primary user-facing message body.
	pub message: &'a str,
	/// Optional contextual detail. (Please ensure this does not contain sensitive information. You can use the Secret type in gmn_core::secret to redact sensitive data if needed.)
	pub context: Option<&'a str>,
	/// Optional actionable hint.
	pub hint: Option<&'a str>,
}

impl<'a> DisplayMetadata for DisplayMessage<'a> {
	fn title(&self) -> &str {
		self.title
	}

	fn code(&self) -> &str {
		self.code
	}

	fn message(&self) -> String {
		self.message.to_string()
	}

	fn context(&self) -> Option<String> {
		self.context.map(ToString::to_string)
	}

	fn hint(&self) -> Option<&str> {
		self.hint
	}
}

impl DisplayMetadata for GmnError {
	fn title(&self) -> &str {
		match self {
			GmnError::Generic(_) => "Error",
			GmnError::Config(_) => "Configuration Error",
			GmnError::Tracing(_) => "Tracing Error",
			GmnError::CLI(_) => "CLI Error",
			GmnError::Database(_) => "Database Error",
			GmnError::Auth(_) => "Authentication Error",
			GmnError::RateLimit(_) => "Rate Limit Exceeded",
			GmnError::Api(_) => "API Error",
			GmnError::Internal(_) => "Internal Error",
		}
	}

	fn code(&self) -> &str {
		self.code()
	}

	fn message(&self) -> String {
		self.to_string()
	}

	fn context(&self) -> Option<String> {
		self.context()
	}

	fn hint(&self) -> Option<&str> {
		self.hint()
	}
}

/// Generic adapter for any standard error where no richer metadata exists.
///
/// Provides sensible defaults:
/// - title: `Error`
/// - code: `ERR-000`
/// - message: `error.to_string()`
#[derive(Debug)]
pub struct GenericErrorDisplay<'a, E: std::error::Error + ?Sized> {
	/// The underlying error being displayed.
	pub error: &'a E,
	/// Human-readable title shown in the message header.
	pub title: &'a str,
	/// Stable code identifier (e.g. `ERR-000`).
	pub code: &'a str,
	/// Optional contextual detail. (Please ensure this does not contain sensitive information. You can use the Secret type in gmn_core::secret to redact sensitive data if needed.)
	pub context: Option<&'a str>,
	/// Optional actionable hint.
	pub hint: Option<&'a str>,
}

impl<'a, E: std::error::Error + ?Sized> GenericErrorDisplay<'a, E> {
	/// Create a new display adapter for a standard error with default metadata.
	pub fn new(error: &'a E) -> Self {
		Self { error, title: "Error", code: "ERR-000", context: None, hint: None }
	}

	/// Builder methods for overriding default metadata.
	pub fn with_title(mut self, title: &'a str) -> Self {
		self.title = title;
		self
	}

	/// Builder method for overriding the default code.
	pub fn with_code(mut self, code: &'a str) -> Self {
		self.code = code;
		self
	}

	/// Builder method for adding optional context.
	pub fn with_context(mut self, context: &'a str) -> Self {
		self.context = Some(context);
		self
	}

	/// Builder method for adding an optional hint.
	pub fn with_hint(mut self, hint: &'a str) -> Self {
		self.hint = Some(hint);
		self
	}
}

impl<'a, E: std::error::Error + ?Sized> DisplayMetadata for GenericErrorDisplay<'a, E> {
	fn title(&self) -> &str {
		self.title
	}

	fn code(&self) -> &str {
		self.code
	}

	fn message(&self) -> String {
		self.error.to_string()
	}

	fn context(&self) -> Option<String> {
		self.context.map(ToString::to_string)
	}

	fn hint(&self) -> Option<&str> {
		self.hint
	}
}

#[track_caller]
fn display_with_level<T: DisplayMetadata + ?Sized>(level: PrettyMessageLevel, data: &T) {
	let title = data.title();
	let code = data.code();
	let message = data.message();
	let context = data.context();
	let hint = data.hint();

	let location = std::panic::Location::caller();
	let location_str = format!("{}:{}", location.file(), location.line());

	match level {
		PrettyMessageLevel::Error => tracing::error!(
			message_code = code,
			message_type = title,
			message = %message,
			context = ?context,
			hint = ?hint,
			location = %location_str,
			"Error displayed to user"
		),
		PrettyMessageLevel::Warning => tracing::warn!(
			message_code = code,
			message_type = title,
			message = %message,
			context = ?context,
			hint = ?hint,
			location = %location_str,
			"Warning displayed to user"
		),
		PrettyMessageLevel::Info => tracing::info!(
			message_code = code,
			message_type = title,
			message = %message,
			context = ?context,
			hint = ?hint,
			location = %location_str,
			"Info message displayed to user"
		),
		PrettyMessageLevel::Success => tracing::info!(
			message_code = code,
			message_type = title,
			message = %message,
			context = ?context,
			hint = ?hint,
			location = %location_str,
			"Success message displayed to user"
		),
		PrettyMessageLevel::Input => tracing::debug!(
			message_code = code,
			message_type = title,
			message = %message,
			context = ?context,
			hint = ?hint,
			location = %location_str,
			"Input message displayed to user"
		),
	}

	print_pretty_message(
		level,
		title,
		code,
		&message,
		context.as_deref(),
		hint,
		Some(&location_str),
	);
}

/// Display an error from any metadata provider.
///
/// This is the generic replacement for the previous `GmnError`-only API.
#[track_caller]
pub fn display_error<T: DisplayMetadata + ?Sized>(data: &T) {
	display_with_level(PrettyMessageLevel::Error, data);
}

/// Display a warning from any metadata provider.
#[track_caller]
pub fn display_warning<T: DisplayMetadata + ?Sized>(data: &T) {
	display_with_level(PrettyMessageLevel::Warning, data);
}

/// Display an info message from any metadata provider.
#[track_caller]
pub fn display_info<T: DisplayMetadata + ?Sized>(data: &T) {
	display_with_level(PrettyMessageLevel::Info, data);
}

/// Display a success message from any metadata provider.
#[track_caller]
pub fn display_success<T: DisplayMetadata + ?Sized>(data: &T) {
	display_with_level(PrettyMessageLevel::Success, data);
}

/// Display a generic `std::error::Error` without requiring `GmnError`.
///
/// Example:
/// `display_std_error(&io_err, Some("File I/O Error"), Some("IO-001"), None, None);`
#[track_caller]
pub fn display_std_error<E: std::error::Error + ?Sized>(
	error: &E,
	title: Option<&str>,
	code: Option<&str>,
	context: Option<&str>,
	hint: Option<&str>,
) {
	let mut msg = GenericErrorDisplay::new(error);
	if let Some(t) = title {
		msg = msg.with_title(t);
	}
	if let Some(c) = code {
		msg = msg.with_code(c);
	}
	if let Some(ctx) = context {
		msg = msg.with_context(ctx);
	}
	if let Some(h) = hint {
		msg = msg.with_hint(h);
	}

	display_error(&msg);
}
