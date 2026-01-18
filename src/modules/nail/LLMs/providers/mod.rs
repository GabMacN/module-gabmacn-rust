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

* Path: src/modules/nail/LLMs/providers/mod.rs
* Name: mod.rs
* Description: Simply re-exports all LLM provider implementations.
*/

#[path = "LLMProvider.rs"]
mod m_llm_provider;
pub use m_llm_provider::LLMProvider;
pub use m_llm_provider::LLMProviderCapabilities;

#[path = "ChutesProvider.rs"]
mod m_chutes_provider;
pub use m_chutes_provider::ChutesLLMProvider;
