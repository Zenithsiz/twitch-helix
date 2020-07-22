//! Helix responses

/// Helix response
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum HelixResponse<T> {
	/// Ok
	Ok(T),

	/// Error
	Err {
		/// Error
		error: Option<String>,

		/// Status
		status: usize,

		/// Message
		message: String,
	},
}

/// Response pagination
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Pagination {
	/// Current cursor
	cursor: String,
}
