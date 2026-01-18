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

* Path: src/core/errors/mod.rs
* Name: mod.rs
* Description: The error handling module for the core component of the GabMacN ecosystem.
  This module defines and manages various error types and handling mechanisms to ensure
  robust and reliable operation across the core functionalities.
*/

// External Crates
use std::fmt;
use thiserror::Error;

// Internal Modules
mod registry;
use registry::get_error_details;

#[allow(non_snake_case)] // Module is used only to extract pretty-print helpers
mod printPrettyError;
pub use printPrettyError::{
  PrettyMessageLevel, pretty_message_to_string, print_pretty_error, print_pretty_info,
  print_pretty_input, print_pretty_message, print_pretty_success, print_pretty_warning,
};

// Error Codes
pub mod codes;

/*
*
* MAIN CODE
*
*/

// Router Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum GMNErrorKind {
  #[error("Core error: {0:?}")]
  Core(codes::GMNCoreErrorCode),
}

impl GMNErrorKind {
  // Routes the request to the correct sub-module to get the JSON key
  pub fn key(&self) -> &'static str {
    match self {
      GMNErrorKind::Core(c) => c.as_key(),
    }
  }
}

#[derive(Debug)]
pub struct GMNError {
  pub kind: GMNErrorKind,
  pub hint: Option<String>,    // Optional hint for resolution
  pub context: Option<String>, // Dynamic data (filenames, IDs, etc.)

  pub location: String, // Future use: file/line info
}

impl GMNError {
  // Helper constructor for Core errors
  #[track_caller]
  pub fn core(code: codes::GMNCoreErrorCode, hint: Option<&str>, context: Option<&str>) -> Self {
    let caller = std::panic::Location::caller();
    Self {
      kind: GMNErrorKind::Core(code),
      hint: hint.map(|s| s.into()),
      context: context.map(|s| s.into()),
      location: format!("{}:{}:{}", caller.file(), caller.line(), caller.column()),
    }
  }

  pub fn pretty_print(&self) {
    // 1. Ask the nested enums for the JSON key (e.g., "GMN_LLM_001")
    let key = self.kind.key();

    // 2. Lookup in JSON
    let def = get_error_details(key).unwrap();

    // 3. Print
    print_pretty_error(
      &def.title,
      key,
      &def.message,
      self.context.as_deref(),
      self.hint.as_deref().or(def.hint.as_deref()),
      Some(&self.location),
    );
  }
}

impl fmt::Display for GMNError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let key = self.kind.key();
    let Some(def) = get_error_details(key) else {
      return write!(f, "{}: {:?}", key, self.context);
    };

    match pretty_message_to_string(
      PrettyMessageLevel::Error,
      &def.title,
      key,
      &def.message,
      self.context.as_deref(),
      self.hint.as_deref().or(def.hint.as_deref()),
      Some(&self.location),
    ) {
      Ok(rendered) => f.write_str(&rendered),
      Err(_) => Err(fmt::Error),
    }
  }
}

impl std::error::Error for GMNError {}

/// Trait implemented by all error-code enums that can be converted into a `GMNError`.
/// This makes the `gmn_expect!` macro extensible to future namespaces beyond `GMNCore`.
pub trait IntoGMNError {
  fn into_gmn_error(self, hint: Option<&str>, context: Option<&str>) -> GMNError;
}

impl IntoGMNError for codes::GMNCoreErrorCode {
  fn into_gmn_error(self, hint: Option<&str>, context: Option<&str>) -> GMNError {
    GMNError::core(self, hint, context)
  }
}

impl IntoGMNError for GMNError {
  fn into_gmn_error(self, _hint: Option<&str>, _context: Option<&str>) -> GMNError {
    self
  }
}

#[macro_export]
macro_rules! gmn_expect {
  (@maybe) => {
    None
  };
  (@maybe $value:expr) => {
    Some($value)
  };
  ($expr:expr, $code:expr $(, hint = $hint:expr)? $(, context = $context:expr)? ) => {{
    let hint_opt: Option<&str> = $crate::gmn_expect!(@maybe $( $hint )?);
    let context_opt: Option<&str> = $crate::gmn_expect!(@maybe $( $context )?);
    match $expr {
      ::std::result::Result::Ok(value) => value,
      ::std::result::Result::Err(_) => panic!(
        "{}",
        $crate::errors::IntoGMNError::into_gmn_error($code, hint_opt, context_opt)
      ),
    }
  }};
}
