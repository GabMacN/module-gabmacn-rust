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

* Path: src/core/errors/registry.rs
* Name: registry.rs
* Description: Manages the error registry for the GabMacN ecosystem,
  loading error definitions from a JSON file and providing lookup functionality.
*/

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

// 1. Define the structure of the JSON data
#[derive(Debug, Deserialize)]
pub struct ErrorDefinition {
  pub title: String,
  pub message: String,
  pub hint: Option<String>,
}

// 2. Define the Type for the Global Registry
type Registry = HashMap<String, ErrorDefinition>;

// 3. Create the Global Static Variable
static ERROR_REGISTRY: OnceLock<Registry> = OnceLock::new();

// 4. The loading function
// This reads the JSON file *at compile time* and parses it *at first runtime use*
fn get_registry() -> &'static Registry {
  ERROR_REGISTRY.get_or_init(|| {
    let json_content = include_str!("errors.json"); // Bakes file into binary
    serde_json::from_str(json_content).expect("Failed to parse errors.json")
  })
}

// 5. The Public Lookup API
pub fn get_error_details(code: &str) -> Option<&'static ErrorDefinition> {
  get_registry().get(code)
}
