# NAIL

This is a document I am making to keep track for myself. Will later update to reflect docs.

The NAIL module stands for the Normalized AI Layer, it is meant to be used as a module that abstracts away different providers and hides them all behind a global Rust API.

## LLMs

In the LLMs folder there are three distinct sections:

1) api, which dictates the common API used by all providers
2) providers, which shows the available implement providers (where custom ones can be implemented).
3) schemas, which shows the translation from the common API to the provider's JSON.
