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

* Path: src/modules/nail/LLMs/providers/ChutesProvider.rs
* Name: ChutesProvider.rs
* Description: The ChutesProvider LLMProvider implementation within the LLMs module of the nail
  module in GabMacN ecosystem.
*/

use crate::modules::nail::llms::api;

use super::super::schemas;

use super::{LLMProvider, LLMProviderCapabilities};

pub struct ChutesLLMProvider {
  /// "Chutes"
  name: String,
  capabilities: LLMProviderCapabilities,
  schema: schemas::LLMSchema,

  /// The URL endpoint for the Chutes LLM service.
  endpoint: String,

  /// The API key for authenticating with the Chutes LLM service. Can only be set at initialization.
  api_key: String,
}

impl ChutesLLMProvider {
  pub fn new(api_key: String, endpoint: String) -> Self {
    ChutesLLMProvider {
      name: "Chutes".to_string(),
      capabilities: LLMProviderCapabilities::TOOLS,
      schema: schemas::LLMSchema::Chutes,
      api_key,
      endpoint,
    }
  }
}

impl LLMProvider for ChutesLLMProvider {
  fn name(&self) -> &str {
    &self.name
  }

  fn capabilities(&self) -> &LLMProviderCapabilities {
    &self.capabilities
  }

  fn set_capabilities(&mut self, capabilities: LLMProviderCapabilities) {
    self.capabilities = capabilities;
  }

  fn schema(&self) -> &schemas::LLMSchema {
    &self.schema
  }

  fn set_schema(&mut self, schema: schemas::LLMSchema) {
    self.schema = schema;
  }
}
