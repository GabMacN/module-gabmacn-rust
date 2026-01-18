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

* Path: src/modules/nail/LLMs/schemas/OpenAI/mod.rs
* Name: mod.rs
* Description: The OpenAI adapter module within the adapters module of the LLMs module
  in the nail module of GabMacN ecosystem. This module provides the implementation to interface
  with OpenAI's large language model APIs, enabling seamless integration and communication
  between the GabMacN ecosystem and OpenAI services.
*/

use crate::types::nail::llms;
pub mod types;

//----------------------------------------------------------------------------
// Conversion Implementations
//----------------------------------------------------------------------------

// Convert between llms::Role and types::OpenAIRole
impl From<llms::Role> for types::OpenAIRole {
  fn from(role: llms::Role) -> Self {
    match role {
      llms::Role::User => types::OpenAIRole::User,
      llms::Role::Assistant => types::OpenAIRole::Assistant,
      llms::Role::System => types::OpenAIRole::System,
      llms::Role::Tool => types::OpenAIRole::Tool,
    }
  }
}

impl From<types::OpenAIRole> for llms::Role {
  fn from(role: types::OpenAIRole) -> Self {
    match role {
      types::OpenAIRole::User => llms::Role::User,
      types::OpenAIRole::Assistant => llms::Role::Assistant,
      types::OpenAIRole::System => llms::Role::System,
      types::OpenAIRole::Tool => llms::Role::Tool,
    }
  }
}
