//! Error handling example
//!
//! This example demonstrates the error handling system and pretty error display.
//!
//! Run with:
//! ```bash
//! cargo run --example error_handling
//! ```

use gmn_core::prelude::*;
use gmn_core::error_display::{display_error, display_warning, display_info, display_success};
use gmn_core::errors::{ConfigError, TracingError, DatabaseError, AuthError};

fn main() -> Result<()> {
	// Initialize tracing
	init_tracing()?;

	info!("Demonstrating error handling");

	// Example 1: Configuration error
	demonstrate_config_error();

	// Example 2: Tracing error
	demonstrate_tracing_error();

	// Example 3: Database error (placeholder)
	demonstrate_database_error();

	// Example 4: Auth error (placeholder)
	demonstrate_auth_error();

	// Example 5: Display utilities
	demonstrate_display_utilities();

	Ok(())
}

fn demonstrate_config_error() {
	info!("=== Configuration Error Example ===");

	let error: GmnError = ConfigError::InvalidLogLevel {
		level: "invalid_level".to_string(),
	}
	.into();

	display_error(&error);
}

fn demonstrate_tracing_error() {
	info!("=== Tracing Error Example ===");

	let error: GmnError = TracingError::AlreadyInitialized.into();
	display_error(&error);
}

fn demonstrate_database_error() {
	info!("=== Database Error Example ===");

	let error: GmnError = DatabaseError::ConnectionFailed(
		"Failed to connect to database at localhost:5432".to_string(),
	)
	.into();

	display_error(&error);
}

fn demonstrate_auth_error() {
	info!("=== Authentication Error Example ===");

	let error: GmnError = AuthError::InvalidCredentials.into();
	display_error(&error);
}

fn demonstrate_display_utilities() {
	info!("=== Display Utilities Example ===");

	// Warning
	display_warning(
		"Deprecated API",
		"GMN-WARN-001",
		"You are using a deprecated API endpoint",
		Some("This endpoint will be removed in version 2.0"),
		Some("Use the new /v2/api endpoint instead"),
	);

	// Info
	display_info(
		"System Status",
		"GMN-INFO-001",
		"All systems operational",
		Some("Last checked: 2026-02-09 00:30:00"),
		None,
	);

	// Success
	display_success(
		"Operation Complete",
		"GMN-SUCCESS-001",
		"Database migration completed successfully",
		Some("Migrated 1,234 records in 2.5 seconds"),
		Some("Remember to update your application configuration"),
	);
}
