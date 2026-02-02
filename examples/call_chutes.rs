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

* Path: src/examples/call_chutes.rs
* Name: call_chutes.rs
* Description: Example demonstrating how to use the ChutesProvider LLMProvider within the LLMs module of the nail
	module in GabMacN ecosystem.
*/

use dotenv::dotenv;

use gabmacn::errors;
use gabmacn::gmn_expect;
use gabmacn::modules::nail::llms;
use gabmacn::modules::nail::prelude::*;

fn main() {
	// Load environment variables from a .env file
	dotenv().ok();

	// Retrieve the Chutes API key from environment variables
	let api_key = gmn_expect!(
		std::env::var("CHUTES_API_KEY"),
		errors::codes::GMNCoreErrorCode::ConfigurationError,
		hint = "CHUTES_API_KEY must be set in .env file",
		context = "Missing CHUTES_API_KEY environment variable."
	);
	let endpoint = "https://llm.chutes.ai/v1/chat/completions";

	println!("--- Calling Nail Module API Example ---");

	let my_chutes_provider = llms::providers::ChutesLLMProvider::new(api_key, endpoint.to_string());
	println!("Using LLM Provider: {}", my_chutes_provider.name());

	let my_agent =
		llms::api::ChatAgent::new("ExampleAgent".to_string(), Box::new(my_chutes_provider));
	println!("Created Agent: {}", my_agent.name());
}
