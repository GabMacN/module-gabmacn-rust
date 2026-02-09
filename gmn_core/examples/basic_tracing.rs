//! Basic tracing example
//!
//! This example demonstrates how to initialize tracing and use basic logging.
//!
//! Run with:
//! ```bash
//! cargo run --example basic_tracing
//! ```
//!
//! Try different log levels:
//! ```bash
//! GMN_LOG_LEVEL=debug cargo run --example basic_tracing
//! GMN_LOG_FORMAT=json cargo run --example basic_tracing
//! ```

use gmn_core::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
	// Initialize tracing with default configuration (from environment)
	init_tracing()?;

	info!("Application started");
	debug!("This is a debug message");
	trace!("This is a trace message (usually not shown)");

	// Example with structured fields
	info!(
		user_id = 12345,
		action = "login",
		"User performed an action"
	);

	// Simulate some work
	perform_operation();

	// Example with span
	let span = trace_operation!("main_operation", operation_id = "op-001");
	let _guard = span.enter();
	info!("Inside the main operation span");

	// Nested operation
	nested_operation();

	info!("Application finished");
	Ok(())
}

fn perform_operation() {
	let _span = trace_operation!("perform_operation");
	info!("Performing some operation");
	thread::sleep(Duration::from_millis(100));
	warn!("This is a warning message");
}

fn nested_operation() {
	let _span = trace_operation!("nested_operation", depth = 2);
	info!("This is a nested operation");
	error!("This is an error message (but not a real error)");
}
