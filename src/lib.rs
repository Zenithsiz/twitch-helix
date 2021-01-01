//! Twitch Helix requests
//!
//! This library offers a list of requests supported by twitch's `Helix`API.
//!
//! Each request is in it's own submodule, under the [`requests`] module,
//! typically following it's url, after `/helix/...`, where possible.
//!
//! # Client
//! This library also offers a client to help with the connections.
//! It is not mandatory to use the client, as all requests do not depend
//! on it, however.

// Lints
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]
// We disable the lints we do not care for
#![allow(clippy::blanket_clippy_restriction_lints)]
// We have fine-grained modules, which causes this to happen often
#![allow(clippy::module_name_repetitions)]
// No need to mark EVERY public function as `inline`
#![allow(clippy::missing_inline_in_public_items)]
// We prefer implicit returns
#![allow(clippy::implicit_return)]
// We shadow variables, as long as they have the same meaning
#![allow(clippy::shadow_reuse)]
// We have fine-grained error types, which are self-explanatory
#![allow(clippy::missing_errors_doc)]
// False positive, we have them all
#![allow(clippy::cargo_common_metadata)]
// We prefer to use match ergonomics
#![allow(clippy::pattern_type_mismatch)]

// Modules
#[cfg(feature = "client")]
pub mod client;
pub mod request;
pub mod response;
#[macro_use]
pub mod url;
mod util;

// Exports
#[cfg(feature = "client")]
pub use client::Client;
pub use request::{HelixRequest, HttpMethod, OAuthRequest};
pub use response::{HelixResponse, OAuthResponse};
