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

* Path: src/modules/nail/LLMs/schemas/Standard/types.rs
* Name: types.rs
* Description: Types definitions for the Standard adapter within the adapters module
  of the LLMs module in the nail module of GabMacN ecosystem. This file contains data structures
  and types specific to interfacing with standard LLM APIs.
*/

use serde::{Deserialize, Serialize};
use serde_json::Value; // Needed for dynamic "parameters" JSON schema

/*
* // MARK: Request
*  ____                            _
* |  _ \ ___  __ _ _   _  ___  ___| |_
* | |_) / _ \/ _` | | | |/ _ \/ __| __|
* |  _ <  __/ (_| | |_| |  __/\__ \ |_
* |_| \_\___|\__, |\__,_|\___||___/\__|
*               |_|
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StandardChatRequest {
  /// The model ID (e.g., "llama-3-70b", "mixtral-8x7b")
  pub model: String,

  /// The conversation history.
  pub messages: Vec<StandardMessage>,

  /* --- Core Generation Parameters --- */
  /// Temperature controls randomness in generation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temperature: Option<f32>,

  /// Top-p (nucleus) sampling parameter.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_p: Option<f32>,

  /// Maximum tokens to generate in the response.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_tokens: Option<u32>,

  /// Whether to stream the response.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stream: Option<bool>,

  /// Penalties are widely supported by vLLM/Llama.cpp backends
  #[serde(skip_serializing_if = "Option::is_none")]
  pub frequency_penalty: Option<f32>,

  /// Presence penalty is also widely supported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub presence_penalty: Option<f32>,

  /// Stop sequences are standard across almost all LLMs.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop: Option<StopSequence>,

  /// Random seed for reproducibility.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub seed: Option<i64>,

  /* --- Tools & JSON --- */
  /// Tools are now standard (Groq, Together, Mistral all support this schema).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tools: Option<Vec<StandardToolDefinition>>,

  /// Tool choice control.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_choice: Option<StandardToolChoice>,

  /// JSON Mode support.
  /// CAUTION: "json_schema" (Structured Outputs) is NOT standard yet.
  /// Only "json_object" is safe to send everywhere.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_format: Option<StandardResponseFormat>,
}

/*
* // MARK: Messages
*  __  __
* |  \/  | ___  ___ ___  __ _  __ _  ___  ___
* | |\/| |/ _ \/ __/ __|/ _` |/ _` |/ _ \/ __|
* | |  | |  __/\__ \__ \ (_| | (_| |  __/\__ \
* |_|  |_|\___||___/___/\__,_|\__, |\___||___/
*                             |___/
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StandardMessage {
  pub role: StandardRole,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  // ^ CHANGED: Kept simple String for "Standard".
  // Most OSS models choke on array-based content (Multimodal) unless specifically multimodal.
  // If you need images, use a separate adapter.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub tool_calls: Vec<StandardToolCall>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StandardRole {
  System,
  User,
  Assistant,
  Tool,
}

/*
* // MARK: Tool Def
*  _____           _       ____        __
* |_   _|__   ___ | |___  |  _ \  ___ / _|
*   | |/ _ \ / _ \| / __| | | | |/ _ \ |_
*   | | (_) | (_) | \__ \ | |_| |  __/  _|
*   |_|\___/ \___/|_|___/ |____/ \___|_|
*
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardToolDefinition {
  #[serde(rename = "type")]
  pub kind: String, // Always "function"
  pub function: StandardFunctionDef,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardFunctionDef {
  pub name: String,
  pub description: Option<String>,
  pub parameters: Value, // Standard JSON Schema
                         // REMOVED: strict (OpenAI only)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StandardToolChoice {
  Simple(String), // "auto", "none", "required"
  Specific(StandardToolChoiceSpecific),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardToolChoiceSpecific {
  #[serde(rename = "type")]
  pub kind: String, // "function"
  pub function: ToolFuncName,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolFuncName {
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardToolCall {
  pub id: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub function: StandardFunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardFunctionCall {
  pub name: String,
  pub arguments: String,
}

/*
* // MARK: Utils
*  _   _ _   _ _ _ _   _
* | | | | |_(_) (_) |_(_) ___  ___
* | | | | __| | | | __| |/ _ \/ __|
* | |_| | |_| | | | |_| |  __/\__ \
*  \___/ \__|_|_|_|\__|_|\___||___/
*
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StopSequence {
  Single(String),
  Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StandardResponseFormat {
  /// Safe everywhere.
  Text,
  /// Supported by Groq, Together, vLLM, Ollama.
  JsonObject,
  // REMOVED: JsonSchema (Use a specialized adapter if you need Strict Structured Outputs)
}
