//! Advanced instrumentation example
//!
//! This example demonstrates advanced instrumentation patterns including
//! domain-specific tracing, performance measurement, and span management.
//!
//! Run with:
//! ```bash
//! GMN_LOG_LEVEL=debug cargo run --example instrumentation
//! ```

use gmn_core::prelude::*;
use gmn_core::domains::{api, auth, database, rate_limit};
use gmn_core::config::TracingConfig;
use std::time::Instant;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
	// Initialize with development config for better visibility
	let config = TracingConfig::development();
	init_tracing_with_config(config)?;

	info!("Starting advanced instrumentation demo");

	// Database operations
	demonstrate_database_tracing();

	// API operations
	demonstrate_api_tracing();

	// Authentication operations
	demonstrate_auth_tracing();

	// Rate limiting
	demonstrate_rate_limit_tracing();

	// Performance measurement
	demonstrate_performance_measurement();

	info!("Advanced instrumentation demo complete");
	Ok(())
}

fn demonstrate_database_tracing() {
	info!("=== Database Tracing Demo ===");

	// Query operation
	let span = database::query_span(database::DbOperation::Select, "users");
	let _guard = span.enter();

	info!("Executing SELECT query on users table");
	thread::sleep(Duration::from_millis(50));

	// Record metrics
	database::record_query_metrics(&span, 42, 50);
	info!("Query returned 42 rows in 50ms");
}

fn demonstrate_api_tracing() {
	info!("=== API Tracing Demo ===");

	// API request
	let span = api::request_span_with_id(
		api::HttpMethod::Get,
		"/api/v1/users",
		"req-12345",
	);
	let _guard = span.enter();

	info!("Processing API request");
	thread::sleep(Duration::from_millis(100));

	// Record completion
	api::record_request_completion(&span, 200, 100);
	info!("Request completed with status 200");

	// External API call
	drop(_guard);
	let ext_span = api::external_api_span("github", "/api/v3/users/octocat");
	let _ext_guard = ext_span.enter();

	info!("Calling external GitHub API");
	thread::sleep(Duration::from_millis(200));
	api::record_request_completion(&ext_span, 200, 200);
}

fn demonstrate_auth_tracing() {
	info!("=== Authentication Tracing Demo ===");

	// Login operation
	let span = auth::auth_span(auth::AuthOperation::Login, Some("user-123"));
	let _guard = span.enter();

	info!("Processing login request");
	thread::sleep(Duration::from_millis(75));

	auth::record_auth_result(&span, true, 75);
	info!("Login successful");

	// API key validation
	drop(_guard);
	let key_span = auth::api_key_span("gmn_live_abc");
	let _key_guard = key_span.enter();

	info!("Validating API key");
	thread::sleep(Duration::from_millis(25));
	auth::record_api_key_result(&key_span, true, 25);
}

fn demonstrate_rate_limit_tracing() {
	info!("=== Rate Limiting Tracing Demo ===");

	// Rate limit check
	let span = rate_limit::rate_limit_check_span("api_requests", "user-123");
	let _guard = span.enter();

	info!("Checking rate limit");

	rate_limit::record_check_result(
		&span,
		rate_limit::RateLimitResult::Allowed,
		45,  // requests
		100, // limit
		60,  // window_secs
		55,  // remaining
	);
	info!("Rate limit check passed");

	// Simulate rate limit exceeded
	drop(_guard);
	rate_limit::log_rate_limit_exceeded("api_requests", "user-456", 101, 100);
}

fn demonstrate_performance_measurement() {
	info!("=== Performance Measurement Demo ===");

	// Using the measure_duration macro
	let result = measure_duration!("expensive_computation", {
		info!("Starting expensive computation");
		thread::sleep(Duration::from_millis(150));
		42
	});

	info!(result = result, "Computation complete");

	// Manual measurement with span
	let span = trace_operation!("manual_measurement");
	let _guard = span.enter();
	let start = Instant::now();

	info!("Performing manual measurement");
	thread::sleep(Duration::from_millis(100));

	let duration = start.elapsed();
	info!(duration_ms = duration.as_millis(), "Manual measurement complete");
}
