//! Database operation tracing utilities.
//!
//! This module provides tracing helpers specifically for database operations.

use tracing::Span;

/// Database operation types
#[derive(Debug, Clone, Copy)]
pub enum DbOperation {
	/// SELECT query
	Select,
	/// INSERT query
	Insert,
	/// UPDATE query
	Update,
	/// DELETE query
	Delete,
	/// Transaction begin
	BeginTransaction,
	/// Transaction commit
	CommitTransaction,
	/// Transaction rollback
	RollbackTransaction,
	/// Connection pool operation
	PoolOperation,
}

impl DbOperation {
	/// Get the string representation of the operation
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Select => "SELECT",
			Self::Insert => "INSERT",
			Self::Update => "UPDATE",
			Self::Delete => "DELETE",
			Self::BeginTransaction => "BEGIN",
			Self::CommitTransaction => "COMMIT",
			Self::RollbackTransaction => "ROLLBACK",
			Self::PoolOperation => "POOL",
		}
	}
}

/// Create a span for a database query operation
pub fn query_span(operation: DbOperation, table: &str) -> Span {
	tracing::info_span!(
		"db_query",
		operation = operation.as_str(),
		table = table,
		rows_affected = tracing::field::Empty,
		duration_ms = tracing::field::Empty,
	)
}

/// Create a span for a database transaction
pub fn transaction_span(operation: DbOperation) -> Span {
	tracing::info_span!(
		"db_transaction",
		operation = operation.as_str(),
		duration_ms = tracing::field::Empty,
	)
}

/// Create a span for connection pool operations
pub fn pool_span(operation: &str) -> Span {
	tracing::info_span!(
		"db_pool",
		operation = operation,
		active_connections = tracing::field::Empty,
		idle_connections = tracing::field::Empty,
	)
}

/// Record query execution metrics
pub fn record_query_metrics(span: &Span, rows_affected: u64, duration_ms: u64) {
	span.record("rows_affected", rows_affected);
	span.record("duration_ms", duration_ms);
}

/// Record connection pool metrics
pub fn record_pool_metrics(span: &Span, active: u32, idle: u32) {
	span.record("active_connections", active);
	span.record("idle_connections", idle);
}
