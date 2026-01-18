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

* Path: src/modules/nail/LLMs/api/ChatAgent.rs
* Name: ChatAgent.rs
* Description: Defines the ChatAgent struct.
*/

use super::super::{
  super::{providers, schemas},
  LLMAgent,
};

pub struct ChatAgent {
  /// The name of the agent.
  name: String,
  /// The LLM provider associated with this agent.
  llm_provider: Box<dyn providers::LLMProvider>,
}

impl ChatAgent {
  /// Creates a new ChatAgent with the given name and LLM provider.
  pub fn new(name: String, llm_provider: Box<dyn providers::LLMProvider>) -> Self {
    Self { name, llm_provider }
  }
}

impl LLMAgent for ChatAgent {
  fn name(&self) -> &str {
    &self.name
  }

  fn llm_provider(&self) -> &Box<dyn providers::LLMProvider> {
    &self.llm_provider
  }

  fn set_llm_provider(&mut self, provider: Box<dyn providers::LLMProvider>) {
    self.llm_provider = provider;
  }
}
