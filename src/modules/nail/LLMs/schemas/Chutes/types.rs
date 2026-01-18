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

* Path: src/modules/nail/LLMs/schemas/Chutes/types.rs
* Name: types.rs
* Description: Types definitions for the Chutes adapter within the nail module of GabMacN
  ecosystem. This module defines data structures and types specific to the Chutes LLM
  adapter, facilitating communication and interaction with the Chutes API.
*/

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/*
* // MARK: Chutes Request
*  ____                            _
* |  _ \ ___  __ _ _   _  ___  ___| |_
* | |_) / _ \/ _` | | | |/ _ \/ __| __|
* |  _ <  __/ (_| | |_| |  __/\__ \ |_
* |_| \_\___|\__, |\__,_|\___||___/\__|
*               |_|
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChutesChatRequest {
  /// The model ID (e.g., "hermes-4-405b", "deepseek-v3")
  pub model: String,

  /// The conversation history.
  pub messages: Vec<ChutesMessage>,

  /* --- Core Sampling Parameters --- */
  /// Temperature controls randomness. Default 0.7.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temperature: Option<f32>,

  /// Top-p (nucleus) sampling. Default 1.0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_p: Option<f32>,

  /// Top-k sampling. Default -1 (disabled).
  /// Used to limit the pool of tokens to the top K probabilities.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_k: Option<i32>,

  /// Min-p sampling. Default 0.
  /// Sets a threshold probability relative to the most likely token.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_p: Option<f32>,

  /* --- Penalties & Bias --- */
  /// Frequency penalty. Default 0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub frequency_penalty: Option<f32>,

  /// Presence penalty. Default 0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub presence_penalty: Option<f32>,

  /// Repetition penalty. Default 1.0 (no penalty).
  /// Classic vLLM parameter, different from freq/presence penalty.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub repetition_penalty: Option<f32>,

  /// Length penalty. Default 1.0.
  /// Used during beam search or to encourage longer/shorter outputs.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub length_penalty: Option<f32>,

  /// Logit bias to modify likelihood of specific tokens.
  /// Maps Token ID (as string) to bias value.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub logit_bias: Option<HashMap<String, f32>>,

  /* --- Output Control --- */
  /// Maximum tokens to generate.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_tokens: Option<u32>,

  /// Minimum tokens to generate. Default 0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_tokens: Option<u32>,

  /// Stop sequences.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop: Option<StopSequence>,

  /// Stop token IDs (Integer IDs).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_token_ids: Option<Vec<u32>>,

  /// Whether to include the stop string in the actual output text.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub include_stop_str_in_output: Option<bool>,

  /// Ignore End-Of-Sequence token. Default false.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ignore_eos: Option<bool>,

  /// Skip special tokens in output. Default true.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub skip_special_tokens: Option<bool>,

  /// Add spaces between special tokens. Default true.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub spaces_between_special_tokens: Option<bool>,

  /* --- Advanced Features --- */
  /// Stream the response. Default false.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stream: Option<bool>,

  /// Random seed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub seed: Option<u64>,

  /// Return logprobs. Default false.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub logprobs: Option<bool>,

  /// Number of top logprobs to return. Default 0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_logprobs: Option<u32>,

  /// Prompt logprobs.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prompt_logprobs: Option<u32>,

  /// Best of N candidates. Used for beam search strategies.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub best_of: Option<u32>,

  /// Use beam search. Default false.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_beam_search: Option<bool>,

  /// Structured output format.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_format: Option<ChutesResponseFormat>,
}

/*
* // MARK: Messages
*  __  __
* |  \/  | ___  ___ ___  __ _  __ _  ___  ___
* | |\/| |/ _ \/ __/ __|/ _` |/ _` |/ _ \/ __|
* | |  | |  __/\__ \__ \ (_| | (_| |  __/\__ \
* |_|  |_|\___||___/___/\__,_|\__, |\___||___/
*                             |___/

* The Chutes schema provided is simpler than OpenAI's.
* It strictly requires `role` and `content`.
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChutesMessage {
  pub role: String, // String allows for flexible roles, though usually "user"/"assistant"
  pub content: String,
}

/*
* // MARK: Utilities
*  _   _ _   _ _ _ _   _
* | | | | |_(_) (_) |_(_) ___  ___
* | | | | __| | | | __| |/ _ \/ __|
* | |_| | |_| | | | |_| |  __/\__ \
*  \___/ \__|_|_|_|\__|_|\___||___/
*
*/

/*
* Response Formats
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChutesResponseFormat {
  Text,
  JsonObject,
  JsonSchema { json_schema: ChutesJsonSchemaDef },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChutesJsonSchemaDef {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub schema: Value, // The actual JSON schema object

  /// Whether to strictly enforce the schema.
  /// Default is usually false for vLLM, unlike OpenAI, which defaults to true.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strict: Option<bool>,
}

/*
* Helpers
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StopSequence {
  Single(String),
  Array(Vec<String>),
}

/*
* Chutes Invocation Wrapper
* If you are calling the generic Chutes function endpoint instead of the
* OpenAI-compatible endpoint, the request needs to be wrapped in `input_args`.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChutesInvocation {
  pub input_args: ChutesChatRequest,
}
