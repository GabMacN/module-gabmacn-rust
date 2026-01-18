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

* Path: src/modules/nail/LLMs/schemas/OpenAI/types.rs
* Name: types.rs
* Description: Types definitions for the OpenAI adapter within the adapters module
  of the LLMs module in the nail module of GabMacN ecosystem. This file contains data structures
  and types specific to interfacing with OpenAI's large language model APIs.
*/

use serde::{Deserialize, Serialize};
use serde_json::Value; // Needed for dynamic "parameters" JSON schema
use std::collections::HashMap;

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
pub struct ChatCompletionRequest {
  /// ID of the model to use (e.g., "gpt-4o")
  pub model: String,

  /// A list of messages comprising the conversation so far.
  pub messages: Vec<OpenAIMessage>,

  /* --- Tools & Structured Outputs --- */
  /// A list of tools the model may call.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tools: Option<Vec<ToolDefinition>>,

  /// Controls which (if any) tool is called by the model.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_choice: Option<ToolChoice>,

  /// Forces the model to output a specific format (JSON mode or JSON Schema).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_format: Option<ResponseFormat>,

  /* --- Generation Parameters --- */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temperature: Option<f32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_p: Option<f32>,

  /// How many chat completion choices to generate for each input message.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub n: Option<u8>,

  /// Up to 4 sequences where the API will stop generating further tokens.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop: Option<Stop>,

  /// The maximum number of tokens to generate in the completion.
  /// Note: o1/reasoning models use `max_completion_tokens` instead.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_tokens: Option<u32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_completion_tokens: Option<u32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub presence_penalty: Option<f32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub frequency_penalty: Option<f32>,

  /// Modify the likelihood of specified tokens appearing in the completion.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub logit_bias: Option<HashMap<String, f32>>,

  /// Whether to stream back partial progress.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stream: Option<bool>,

  /// Options for streaming (e.g., include_usage).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stream_options: Option<StreamOptions>,

  /* --- Metadata --- */
  /// A unique identifier representing your end-user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,

  /// If specified, our system will make a best effort to sample deterministically.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub seed: Option<i64>,

  /// Whether to enable parallel function calling during tool use.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parallel_tool_calls: Option<bool>,

  /// Whether to return log probabilities of the output tokens.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub logprobs: Option<bool>,

  /// An integer between 0 and 20 specifying the number of most likely tokens to return at each position.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub top_logprobs: Option<u8>,

  /// Specifies the latency tier to use (e.g. "auto" or "default")
  #[serde(skip_serializing_if = "Option::is_none")]
  pub service_tier: Option<String>,

  /// Output modalities (e.g. ["text", "audio"]) for GPT-4o Audio
  #[serde(skip_serializing_if = "Option::is_none")]
  pub modalities: Option<Vec<String>>,

  /// Audio configuration for GPT-4o Audio
  #[serde(skip_serializing_if = "Option::is_none")]
  pub audio: Option<AudioConfig>,

  /// Reasoning effort level for o1/reasoning models ("low", "medium", "high")
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reasoning_effort: Option<String>,

  /// Used to speed up generation when content is known in advance.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prediction: Option<PredictionContent>,

  /// Usually this would be an option, but as the writer of this library, I will disable it in my NAIL module by default.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub store: Option<bool>,

  /// Additional metadata to attach to the request.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option<HashMap<String, String>>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
  pub url: String,
  /// "auto", "low", or "high"
  #[serde(skip_serializing_if = "Option::is_none")]
  pub detail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputAudio {
  pub data: String,   // Base64 encoded audio
  pub format: String, // "wav" | "mp3"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
  Text { text: String },
  ImageUrl { image_url: ImageUrl },
  InputAudio { input_audio: InputAudio },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Content {
  /// The standard text-only case.
  Text(String),
  /// The multimodal case (e.g., "Look at this image").
  Parts(Vec<ContentPart>),
}

/// Represents the role of a message in the OpenAI chat format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenAIRole {
  /// The system role, used for setting context.
  System,
  /// The developer role is a new role introduced in the o1 models.
  Developer,
  /// The user role, representing the end-user.
  User,
  /// The assistant role, representing the AI assistant.
  Assistant,
  /// The tool role, representing an external tool or function.
  Tool,
  /// Function is a deprecated role, previously used for function calls.
  Function,
}

/// Represents a message in the OpenAI chat format.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIMessage {
  /// The role of the message (system, user, assistant, tool).
  pub role: OpenAIRole,

  /// The content of the message.
  /// Optional because ToolCalls (Assistant) don't have content
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<Content>,

  /// The name of the author associated with the message.
  /// Optional because only 'user' or 'tool' usually have names in the spec.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  /// Specific to the Assistant role when making a tool call.
  /// Optional because only Assistant messages making tool calls have this field.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub tool_calls: Vec<ToolCallWire>,

  /// The ID of the tool call being referenced.
  /// When role == Tool, this ID matches the `id` from the Assistant's `tool_calls`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool_call_id: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub refusal: Option<String>,
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

/// Controls tool selection.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
  /// "none" | "auto" | "required"
  Simple(String),
  /// Forces a specific function
  Specific(ToolChoiceSpecific),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolChoiceSpecific {
  #[serde(rename = "type")]
  pub kind: String, // "function"
  pub function: ToolChoiceFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolChoiceFunction {
  pub name: String,
}

/// Used in the `tools` list of the Request.
/// Tells the LLM *what* it can do.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolDefinition {
  #[serde(rename = "type")]
  pub kind: String, // Always "function" currently

  /// The definition of the function/tool.
  pub function: FunctionDefinition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDefinition {
  /// The name of the function/tool.
  pub name: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,

  /// This is the JSON Schema Object describing the arguments.
  /// We use serde_json::Value because the schema is dynamic.
  pub parameters: Value,

  /// Optional: Strict mode for structured outputs
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strict: Option<bool>,
}

/*
* // MARK: Tool Calls
*  _____           _       _   _
* |_   _|__   ___ | |___  | | | |___  __ _  __ _  ___
*   | |/ _ \ / _ \| / __| | | | / __|/ _` |/ _` |/ _ \
*   | | (_) | (_) | \__ \ | |_| \__ \ (_| | (_| |  __/
*   |_|\___/ \___/|_|___/  \___/|___/\__,_|\__, |\___|
*                                          |___/
*/

/// Represents a tool call made by the assistant in the OpenAI chat format.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallWire {
  /// The unique identifier of the tool being called.
  pub id: String,

  /// The type of the tool call.
  #[serde(rename = "type")]
  pub kind: String, // usually "function"

  /// The function being called within the tool call.
  pub function: FunctionWire,
}

/// Represents a function called within a tool call in the OpenAI chat format.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionWire {
  /// The name of the function being called.
  pub name: String,

  /// The arguments passed to the function as a JSON string.
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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
  Text,
  JsonObject,
  JsonSchema { json_schema: JsonSchemaDef },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonSchemaDef {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub schema: Value, // The actual JSON schema object
  #[serde(skip_serializing_if = "Option::is_none")]
  pub strict: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Stop {
  Single(String),
  Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamOptions {
  pub include_usage: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
  pub voice: String,  // "alloy", "echo", etc.
  pub format: String, // "wav", "mp3", etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictionContent {
  #[serde(rename = "type")]
  pub kind: String, // "content"
  pub content: Content, // reuse your existing Content enum
}
