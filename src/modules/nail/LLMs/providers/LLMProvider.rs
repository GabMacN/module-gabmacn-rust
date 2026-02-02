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

* Path: src/modules/nail/LLMs/LLMProvider.rs
* Name: LLMProvider.rs
* Description: The LLMProvider trait definition within the LLMs module of the nail
	module in GabMacN ecosystem.
*/

use async_trait::async_trait;
use bitflags::bitflags;

pub use super::super::schemas;

bitflags! {
	/// Flags representing various capabilities of LLM providers.
	pub struct LLMProviderCapabilities: u32 {
		/// Capability to handle tools.
		const TOOLS = 0b00000001;
	}
}

#[async_trait]
pub trait LLMProvider {
	/// The name of the LLM provider.
	fn name(&self) -> &str;

	/// The capabilities of the LLM provider.
	fn capabilities(&self) -> &LLMProviderCapabilities;
	fn set_capabilities(&mut self, capabilities: LLMProviderCapabilities);

	/// The schema used by this LLM provider.
	fn schema(&self) -> &schemas::LLMSchema;
	fn set_schema(&mut self, schema: schemas::LLMSchema);
}
