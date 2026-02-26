//! # Prompt Demo
//!
//! Example demonstrating the use of `gmn_core::prompt` for interactive CLI input.

use gmn_core::error_display::{
	DisplayMessage, display_error, display_info, display_success, display_warning,
};
use gmn_core::errors::{CLIError, GmnError};
use gmn_core::prompt::Prompter;
use gmn_core::tracing::init_tracing;

/// Demo model for a tiny interactive setup flow.
#[derive(Debug)]
struct UserProfile {
	name: String,
	password: String,
	environment: String,
	enable_telemetry: bool,
}

/// Small helper to map `CLIError` into the existing display system through `GmnError`.
fn display_cli_error(err: CLIError) {
	let gmn_error: GmnError = err.into();
	display_error(&gmn_error);
}

fn main() {
	// Best effort tracing initialization for visible structured logs.
	let _ = init_tracing();

	display_info(&DisplayMessage {
		title: "Prompt Demo",
		code: "PROMPT-DEMO-001",
		message: "Starting interactive prompt demonstration.",
		context: Some("This example showcases text, password, select, and confirm prompts."),
		hint: Some("Press Ctrl+C at any prompt to cancel and see cancellation handling."),
	});

	match run_prompt_flow() {
		Ok(profile) => {
			let redacted_password = "*".repeat(profile.password.chars().count().max(8));

			let context_str = format!(
				"name='{}', environment='{}', telemetry={}, password='{}'",
				profile.name, profile.environment, profile.enable_telemetry, redacted_password
			);
			display_success(&DisplayMessage {
				title: "Prompt Flow Complete",
				code: "PROMPT-DEMO-OK",
				message: "User input captured successfully.",
				context: Some(&context_str),
				hint: Some("You can now continue with app bootstrap using these values."),
			});
		}
		Err(err) => match err {
			CLIError::Cancelled { .. } => {
				display_warning(&DisplayMessage {
					title: "Prompt Cancelled",
					code: "PROMPT-DEMO-CANCELLED",
					message: "The interactive flow was cancelled by the user.",
					context: Some("No changes were applied."),
					hint: Some("Run the example again to retry the flow."),
				});
			}
			other => {
				display_cli_error(other);
			}
		},
	}
}

fn run_prompt_flow() -> Result<UserProfile, CLIError> {
	let name = Prompter::text("What is your display name?", Some("gabmacn"), None)?;

	let password = Prompter::password("Enter a password (input hidden)", None)?;
	if password.trim().is_empty() {
		return Err(CLIError::FetchError { message: "Password cannot be empty".to_string() });
	}

	let environment = Prompter::select(
		"Choose target environment",
		vec![
			(0, "development".to_string()),
			(1, "staging".to_string()),
			(2, "production".to_string()),
		],
	)?;

	let enable_telemetry = Prompter::confirm("Enable telemetry?", true)?;

	Ok(UserProfile { name, password, environment: environment.to_string(), enable_telemetry })
}
