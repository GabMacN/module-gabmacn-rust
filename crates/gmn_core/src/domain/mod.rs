//! The Domain module contains the core types and primitives for the application domain.
//!
//! This module exposes **namespaced containers** (for dimensional clarity) while also
//! keeping compatibility re-exports for existing consumers.
//!
//! Example ergonomic paths:
//! - `gmn_core::domain::color::Srgb`
//! - `gmn_core::domain::time::DateTime`
//! - `gmn_core::domain::network::IpAddr`
//!
//! The top-level crate re-exports these, allowing users to get paths (after importing prelude) like:
//! - `Srgb`

// =======================================================================
// Domain Containers (namespaced primitives)
// =======================================================================

/// Color primitives.
pub mod color {
	pub use palette::Srgb;
}

/// Time and date primitives.
pub mod time {
	pub use chrono::{DateTime, NaiveDate, Utc};
}

/// Network primitives.
pub mod network {
	pub use std::net::{IpAddr, SocketAddr};
}

/// Identifier/versioning primitives.
pub mod identity {
	pub use semver::Version;
	pub use uuid::Uuid;
}

/// Security/privacy primitives.
pub mod security {
	pub use secrecy::{
		CloneableSecret, ExposeSecret, ExposeSecretMut, SecretBox, SecretSlice, SecretString,
		zeroize,
	};
}

/// Content sanitization primitives.
pub mod content {
	pub use ammonia::clean as sanitize_html;
}

/// Geometry/spatial crate namespace.
pub mod spatial {
	pub use geo;
}

/// Math/graphics crate namespace.
pub mod math {
	pub use glam;
}

// =======================================================================
// New Type Wrappers
// =======================================================================

// =======================================================================
// Domain Types
// =======================================================================
