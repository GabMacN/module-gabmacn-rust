//! # Prompt
//!
//! Prompt is an adapatation of the `inquire` crate, meant to be used within the GabMacN ecosystem. It provides a simple interface for prompting users for input, with support for various types of prompts (text, password, select, etc.) and customizable validation and formatting.

use super::errors;
use inquire::{Confirm, Password, Select, Text, validator::Validation};
use std::fmt::Display;

/// A utility for prompting users for input in the terminal, with support for various types of prompts and error handling.
#[derive(Debug)]
pub struct Prompter;

#[derive(Clone)]
struct SelectOption<K: Clone> {
	id: K,
	label: String,
}

impl<K: Clone> Display for SelectOption<K> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.label)
	}
}

impl Prompter {
	/// Prompts the user for text input with an optional default value.
	pub fn text(
		prompt: &str,
		default: Option<&str>,
		validator: Option<fn(&str) -> Result<(), String>>,
	) -> Result<String, errors::CLIError> {
		let mut request = Text::new(prompt);

		if let Some(default) = default {
			request = request.with_default(default);
		}

		if let Some(validator) = validator {
			let validator_wrapper = move |input: &str| match validator(input) {
				Ok(_) => Ok(Validation::Valid),
				Err(err_msg) => Ok(Validation::Invalid(err_msg.into())),
			};

			request = request.with_validator(validator_wrapper);
		}

		request.prompt().map_err(|e| Self::map_err(e))
	}

	/// Prompts the user for password input, hiding the input as it's typed.
	pub fn password(
		prompt: &str,
		validator: Option<fn(&str) -> Result<(), String>>,
	) -> Result<String, errors::CLIError> {
		let mut request = Password::new(prompt);

		if let Some(validator) = validator {
			let validator_wrapper = move |input: &str| match validator(input) {
				Ok(_) => Ok(Validation::Valid),
				Err(err_msg) => Ok(Validation::Invalid(err_msg.into())),
			};

			request = request.with_validator(validator_wrapper);
		}

		request.prompt().map_err(|e| Self::map_err(e))
	}

	/// Prompts the user for a yes/no confirmation, with an optional default value.
	pub fn confirm(prompt: &str, default: bool) -> Result<bool, errors::CLIError> {
		Confirm::new(prompt).with_default(default).prompt().map_err(|e| Self::map_err(e))
	}

	/// Prompts the user to select one option from a list of options.
	pub fn select<K: Clone>(
		prompt: &str,
		options: Vec<(K, String)>,
	) -> Result<K, errors::CLIError> {
		// Map your raw tuples into our UI-friendly struct
		let items: Vec<SelectOption<K>> =
			options.into_iter().map(|(id, label)| SelectOption { id, label }).collect();

		// Let inquire do the heavy lifting
		let choice = Select::new(prompt, items).prompt().map_err(|e| Self::map_err(e))?;

		// Strip the UI layer off and just return the ID to the app!
		Ok(choice.id)
	}

	fn map_err(err: inquire::InquireError) -> errors::CLIError {
		match err {
			inquire::InquireError::OperationCanceled
			| inquire::InquireError::OperationInterrupted => errors::CLIError::Cancelled { message: None },
			_ => errors::CLIError::FetchError { message: err.to_string() },
		}
	}
}
