//! Helix client

// Imports
use crate::{HelixRequest, HelixResponse, HttpMethod, OAuthRequest, OAuthResponse};
use reqwest as req;

/// A client to make requests to Helix with.
///
/// The client is initialized given an initial oauth string.
/// This allows you to do a [`OAuthRequest`] for validating
/// this token and getting a client id, which may then be used
/// to make [`HelixRequest`]
#[derive(Clone, Debug)]
pub struct Client {
	/// Underlying client
	client: req::Client,

	/// OAuth token
	oauth: String,
}

/// Error type for [`Client::request_oauth`] and [`Client::request_helix`]
#[derive(Debug, thiserror::Error)]
pub enum RequestError {
	/// Unable to send request
	#[error("Unable to send request")]
	Send(#[source] req::Error),

	/// Unable to parse response
	#[error("Unable to parse response")]
	Parse(#[source] req::Error),
}

impl Client {
	/// Creates a new client given an oauth token
	#[must_use]
	pub fn new(oauth: String) -> Self {
		Self {
			client: req::Client::new(),
			oauth,
		}
	}

	/// Performs an OAuth request to twitch
	pub async fn request_oauth<R: OAuthRequest + Send + Sync>(&mut self, request: &R) -> Result<OAuthResponse<R::Response>, RequestError> {
		// Get url
		let url = request.url();

		// Build the request and send it
		let response = self
			.client
			.get(url)
			.header("Authorization", format!("OAuth {}", self.oauth))
			.send()
			.await
			.map_err(RequestError::Send)?;

		// Then parse the response
		let output = response.json().await.map_err(RequestError::Parse)?;

		Ok(output)
	}

	/// Performs a request to Helix
	pub async fn request_helix<R: HelixRequest + Send + Sync>(
		&mut self, request: &R, client_id: &str,
	) -> Result<HelixResponse<R::Response>, RequestError> {
		// Get url
		let url = request.url();

		// Translate the http method
		let method = match request.http_method() {
			HttpMethod::Get => req::Method::GET,
			HttpMethod::Post => req::Method::POST,
		};

		// Build the request and send it
		let response = self
			.client
			.request(method, url)
			.bearer_auth(&self.oauth)
			.header("Client-ID", client_id)
			.send()
			.await
			.map_err(RequestError::Send)?;

		// Then parse the response
		let output = response.json().await.map_err(RequestError::Parse)?;

		Ok(output)
	}
}
