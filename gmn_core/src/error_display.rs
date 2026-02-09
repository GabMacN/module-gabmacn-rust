//! Error display integration with the pretty error printing module.
//!
//! This module bridges the structured error types with the beautiful terminal
//! output provided by the `printPrettyError` module.

use crate::errors::GmnError;
use crate::printPrettyError::{print_pretty_message, PrettyMessageLevel};

/// Display an error using the pretty error printer
///
/// This function takes a `GmnError` and displays it beautifully in the terminal
/// using the existing `printPrettyError` infrastructure.
pub fn display_error(error: &GmnError) {
	let code = error.code();
	let message = error.to_string();
	let context = error.context();
	let hint = error.hint();

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

	print_pretty_message(
		PrettyMessageLevel::Error,
		title,
		code,
		&message,
		context.as_deref(),
		hint,
		None, // location - could be populated with file/line info in the future
	);
}

/// Display a warning using the pretty printer
pub fn display_warning(title: &str, code: &str, message: &str, context: Option<&str>, hint: Option<&str>) {
	print_pretty_message(
		PrettyMessageLevel::Warning,
		title,
		code,
		message,
		context,
		hint,
		None,
	);
}

/// Display an info message using the pretty printer
pub fn display_info(title: &str, code: &str, message: &str, context: Option<&str>, hint: Option<&str>) {
	print_pretty_message(
		PrettyMessageLevel::Info,
		title,
		code,
		message,
		context,
		hint,
		None,
	);
}

/// Display a success message using the pretty printer
pub fn display_success(title: &str, code: &str, message: &str, context: Option<&str>, hint: Option<&str>) {
	print_pretty_message(
		PrettyMessageLevel::Success,
		title,
		code,
		message,
		context,
		hint,
		None,
	);
}
