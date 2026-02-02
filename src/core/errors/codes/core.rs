/*
*  .d8888b.           888      888b     d888                   888b    888
* d88P  Y88b          888      8888b   d8888                   8888b   888
* 888    888          888      88888b.d88888                   88888b  888
* 888         8888b.  88888b.  888Y88888P888  8888b.   .d8888b 888Y88b 888
* 888  88888     "88b 888 "88b 888 Y888P 888     "88b d88P"    888 Y88b888
* 888    888 .d888888 888  888 888  Y8P  888 .d888888 888      888  Y88888
* Y88b  d88P 888  888 888 d88P 888   "   888 888  888 Y88b.    888   Y8888
*  "Y8888P88 "Y888888 88888P"  888       888 "Y888888  "Y8888P 888    Y888

* All bugs and glitches proudly made by @GabMacN.

* @GabMacN 2026

* GitHub: GabMacN
* Discord: gabmacn
* Youtube: @GabMacN
* Instagram: @gabmacn

* Path: src/core/errors/codes/core.rs
* Name: core.rs
* Description: Defines core error types and handling mechanisms for the GabMacN ecosystem.
*/

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum GMNCoreErrorCode {
	#[error("Unknown core error")]
	Unknown,
	#[error("Bad request")]
	BadRequest,
	#[error("Unauthorized")]
	Unauthorized,
	#[error("Forbidden")]
	Forbidden,
	#[error("Resource not found")]
	NotFound,
	#[error("Conflict")]
	Conflict,
	#[error("Validation error")]
	ValidationError,
	#[error("Too many requests")]
	TooManyRequests,
	#[error("Internal server error")]
	InternalServerError,
	#[error("Not implemented")]
	NotImplemented,
	#[error("Bad gateway")]
	BadGateway,
	#[error("Service unavailable")]
	ServiceUnavailable,
	#[error("Configuration error")]
	ConfigurationError,
	#[error("Invalid input")]
	InvalidInput,
	#[error("Operation failed")]
	OperationFailed,
	#[error("Timeout error")]
	TimeoutError,
	#[error("Resource exhausted")]
	ResourceExhausted,
	#[error("Database error")]
	DatabaseError,
	#[error("Serialization error")]
	SerializationError,
	#[error("File system error")]
	FileSystemError,
}

impl GMNCoreErrorCode {
	// Helper to map Kind -> JSON Code
	pub fn as_key(&self) -> &'static str {
		match self {
			GMNCoreErrorCode::Unknown => "GMN_CORE_000",
			GMNCoreErrorCode::ConfigurationError => "GMN_APP_001",
			GMNCoreErrorCode::InvalidInput => "GMN_APP_002",
			GMNCoreErrorCode::OperationFailed => "GMN_APP_003",
			GMNCoreErrorCode::TimeoutError => "GMN_APP_004",
			GMNCoreErrorCode::ResourceExhausted => "GMN_APP_005",
			GMNCoreErrorCode::DatabaseError => "GMN_APP_006",
			GMNCoreErrorCode::SerializationError => "GMN_APP_007",
			GMNCoreErrorCode::FileSystemError => "GMN_APP_008",

			// HTTP Like Errors, we are not sequential here to match HTTP Status Codes for simplicity.
			GMNCoreErrorCode::BadRequest => "GMN_CORE_400",
			GMNCoreErrorCode::Unauthorized => "GMN_CORE_401",
			GMNCoreErrorCode::Forbidden => "GMN_CORE_403",
			GMNCoreErrorCode::NotFound => "GMN_CORE_404",
			GMNCoreErrorCode::Conflict => "GMN_CORE_409",
			GMNCoreErrorCode::ValidationError => "GMN_CORE_422",
			GMNCoreErrorCode::TooManyRequests => "GMN_CORE_429",
			GMNCoreErrorCode::InternalServerError => "GMN_CORE_500",
			GMNCoreErrorCode::NotImplemented => "GMN_CORE_501",
			GMNCoreErrorCode::BadGateway => "GMN_CORE_502",
			GMNCoreErrorCode::ServiceUnavailable => "GMN_CORE_503",
		}
	}
}
