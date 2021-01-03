//! Helix responses

// Modules
pub mod pagination;

// Exports
pub use pagination::Pagination;

/// Helix response
///
/// Every response from Helix may be an error, of type [`ResponseError`],
/// and each successful response, is wrapped within a `data` field, as well
/// as a possible `pagination` field for requests that may search further.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum HelixResponse<T> {
	/// Ok
	Ok(ResponseData<T>),

	/// Error
	Err(ResponseError),
}

impl<T> HelixResponse<T> {
	/// Turns this response into a `Result`
	pub fn into_result(self) -> Result<ResponseData<T>, ResponseError> {
		match self {
			Self::Ok(ok) => Ok(ok),
			Self::Err(err) => Err(err),
		}
	}
}

/// OAuth response
///
/// Every response from OAuth may be an error, of type [`ResponseError`],
/// bit each successful response, unlike [`HelixResponse`], always contains
/// the response type itself.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OAuthResponse<T> {
	/// Ok
	Ok(T),

	/// Error
	Err(ResponseError),
}

impl<T> OAuthResponse<T> {
	/// Turns this response into a `Result`
	pub fn into_result(self) -> Result<T, ResponseError> {
		match self {
			Self::Ok(ok) => Ok(ok),
			Self::Err(err) => Err(err),
		}
	}
}

/// Response data
///
/// The response data for each helix request is wrapped
/// within a `data` field, as well as contain a `pagination`
/// field for requests with multiple pages.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseData<T> {
	/// The actual data
	pub data: T,

	/// Possible pagination for the data
	pub pagination: Option<Pagination>,
}

/// Response error
///
/// Every response may return an error. Errors from
/// twitch always contain a status and message, and
/// optionally a further error type.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(thiserror::Error)]
#[error("{}: {} ({:?})", status, message, error)]
pub struct ResponseError {
	/// Error
	pub error: Option<String>,

	/// Status
	pub status: usize,

	/// Message
	pub message: String,
}
