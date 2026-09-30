//! The outcome vocabulary and its process exit codes — the engine API's, re-exported.
//!
//! [`Status`] moved into `archogen-api` (leaf `API.3.2`, `docs/decisions/decision_engine-api.md`): the CLI's
//! exit code is a projection of the API's response, not a second opinion about it.

pub use archogen_api::Status;
