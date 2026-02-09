//! Error display integration with the pretty error printing module.
//!
//! This module bridges the structured error types with the beautiful terminal
//! output provided by the `printPrettyError` module.
//!
//! All display functions automatically emit corresponding tracing events to ensure
//! that displayed messages are captured in logs for observability.

use crate::errors::GmnError;
use crate::print_pretty_error::{print_pretty_message, PrettyMessageLevel};

/// Display an error using the pretty error printer
///
/// This function takes a `GmnError` and displays it beautifully in the terminal
/// using the existing `printPrettyError` infrastructure.
///
/// **Important**: This function automatically emits a `tracing::error!` event
/// with the error details to ensure the error is captured in logs. This prevents
/// "phantom errors" where users see errors that don't appear in logs.
///
/// The location of the caller is automatically captured and displayed.
#[track_caller]
pub fn display_error(error: &GmnError) {
	let code = error.code();
	let message = error.to_string();
	let context = error.context();
	let hint = error.hint();

	// Capture caller location
	let location = std::panic::Location::caller();
	let location_str = format!("{}:{}", location.file(), location.line());

	// Extract the title from the error variant
	let title = match error {
		GmnError::Config(_) => "Configuration Error",
		GmnError::Tracing(_) => "Tracing Error",
		GmnError::Database(_) => "Database Error",
		GmnError::Auth(_) => "Authentication Error",
		GmnError::RateLimit(_) => "Rate Limit Exceeded",
		GmnError::Api(_) => "API Error",
		GmnError::Internal(_) => "Internal Error",
	};

	// Emit tracing event BEFORE displaying to ensure it's logged
	tracing::error!(
		error_code = code,
		error_type = title,
		message = %message,
		context = ?context,
		hint = ?hint,
		location = %location_str,
		"Error displayed to user"
	);

	print_pretty_message(
		PrettyMessageLevel::Error,
		title,
		code,
		&message,
		context.as_deref(),
		hint,
		Some(&location_str),
	);
}

/// Display a warning using the pretty printer
///
/// Automatically emits a `tracing::warn!` event before displaying.
/// The location of the caller is automatically captured and displayed.
#[track_caller]
pub fn display_warning(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
) {
	// Capture caller location
	let location = std::panic::Location::caller();
	let location_str = format!("{}:{}", location.file(), location.line());

	// Emit tracing event
	tracing::warn!(
		warning_code = code,
		warning_type = title,
		message = message,
		context = ?context,
		hint = ?hint,
		location = %location_str,
		"Warning displayed to user"
	);

	print_pretty_message(
		PrettyMessageLevel::Warning,
		title,
		code,
		message,
		context,
		hint,
		Some(&location_str),
	);
}

/// Display an info message using the pretty printer
///
/// Automatically emits a `tracing::info!` event before displaying.
/// The location of the caller is automatically captured and displayed.
#[track_caller]
pub fn display_info(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
) {
	// Capture caller location
	let location = std::panic::Location::caller();
	let location_str = format!("{}:{}", location.file(), location.line());

	// Emit tracing event
	tracing::info!(
		info_code = code,
		info_type = title,
		message = message,
		context = ?context,
		hint = ?hint,
		location = %location_str,
		"Info message displayed to user"
	);

	print_pretty_message(
		PrettyMessageLevel::Info,
		title,
		code,
		message,
		context,
		hint,
		Some(&location_str),
	);
}

/// Display a success message using the pretty printer
///
/// Automatically emits a `tracing::info!` event before displaying.
/// The location of the caller is automatically captured and displayed.
#[track_caller]
pub fn display_success(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
) {
	// Capture caller location
	let location = std::panic::Location::caller();
	let location_str = format!("{}:{}", location.file(), location.line());

	// Emit tracing event
	tracing::info!(
		success_code = code,
		success_type = title,
		message = message,
		context = ?context,
		hint = ?hint,
		location = %location_str,
		"Success message displayed to user"
	);

	print_pretty_message(
		PrettyMessageLevel::Success,
		title,
		code,
		message,
		context,
		hint,
		Some(&location_str),
	);
}
