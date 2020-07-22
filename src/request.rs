//! Helix request trait

/// A Helix request
pub trait HelixRequest: Send {
	/// Output type
	type Output: for<'de> serde::Deserialize<'de>;

	/// Returns this request's url
	fn url(&self) -> url::Url;

	/// Returns the request's http method
	fn http_method(&self) -> RequestHttpMethod;
}

/// Request http method
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RequestHttpMethod {
	/// `PUT` method
	Put,

	/// `GET` method
	Get,

	/// `POST` method
	Post,
}
