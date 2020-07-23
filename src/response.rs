//! Helix responses

/// Helix response
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
	#[allow(clippy::missing_const_for_fn)] // False positive, we can't use it because `T` might need to be dropped
	pub fn into_result(self) -> Result<ResponseData<T>, ResponseError> {
		match self {
			Self::Ok(ok) => Ok(ok),
			Self::Err(err) => Err(err),
		}
	}
}

/// Response data
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseData<T> {
	/// The actual data
	pub data: T,

	/// Possible pagination for the data
	pub pagination: Option<Pagination>,
}

/// Response error
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

/// Response pagination
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pagination {
	/// Current cursor
	cursor: String,
}
