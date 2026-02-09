//! GabMacN Core Library
//!
//! This is the foundational library for the GabMacN ecosystem in Rust.
//! It provides core functionality including:
//!
//! - **Tracing and Logging**: Comprehensive observability with structured logging
//! - **Error Handling**: Type-safe error handling with rich context
//! - **Pretty Error Display**: Beautiful terminal error messages
//! - **Domain-Specific Utilities**: Helpers for database, auth, API, and rate limiting
//!
//! # Quick Start
//!
//! ```no_run
//! use gmn_core::prelude::*;
//!
//! fn main() -> gmn_core::Result<()> {
//!     // Initialize tracing
//!     init_tracing()?;
//!
//!     // Use tracing
//!     info!("Application started");
//!
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! ## Tracing
//!
//! The library uses the `tracing` ecosystem for structured logging and observability.
//! Configure tracing using environment variables or programmatically:
//!
//! ```no_run
//! use gmn_core::config::TracingConfig;
//! use gmn_core::tracing_setup::init_tracing_with_config;
//!
//! fn main() -> gmn_core::Result<()> {
//!     let config = TracingConfig::development();
//!     init_tracing_with_config(config)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All errors implement the `GmnError` type with error codes, hints, and context:
//!
//! ```no_run
//! use gmn_core::{GmnError, Result};
//! use gmn_core::error_display::display_error;
//!
//! fn example() -> Result<()> {
//!     // Your code here
//!     Ok(())
//! }
//!
//! fn main() {
//!     if let Err(e) = example() {
//!         display_error(&e);
//!     }
//! }
//! ```

// Public modules
pub mod config;
pub mod domains;
pub mod error_display;
pub mod errors;
pub mod instrumentation;
pub mod print_pretty_error;
pub mod tracing_setup;

// Re-exports for convenience
pub use errors::{GmnError, Result};
pub use tracing_setup::{init_tracing, init_tracing_with_config};

/// Prelude module for convenient imports
///
/// Import this module to get access to commonly used items:
///
/// ```
/// use gmn_core::prelude::*;
/// ```
pub mod prelude {
	pub use crate::config::TracingConfig;
	pub use crate::error_display::display_error;
	pub use crate::errors::{GmnError, Result};
	pub use crate::tracing_setup::{init_tracing, init_tracing_with_config};

	// Re-export common tracing macros
	pub use tracing::{debug, error, info, trace, warn};

	// Re-export instrumentation macros
	pub use crate::{log_event, measure_duration, trace_operation};
}
