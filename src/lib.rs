//! Twitch Helix requests
//!
//! This crate offers a list of all requests `twitch`'s `Helix` API offers.
//!
//! It is left to the user to send these requests themselves, as this crate
//! only offers the url and parsing after data has been received.

// Lints
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery)]
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

// Modules
pub mod oauth;
pub mod request;
pub mod response;
pub mod search;
#[macro_use]
pub mod url;
pub mod client;

// Exports
pub use request::{HelixRequest, OAuthRequest};
pub use response::HelixResponse;
