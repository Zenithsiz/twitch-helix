# Twitch Helix API

This library provides a rust API to interact with the
twitch `Helix` API, as described [here](https://dev.twitch.tv/docs/api/reference).

The API is focused around describing each request, with the actual
requests being done either outside the crate, or by the `Client` type.

## Example

The following example shows how to use the library to make a request
to validate an oauth token, receiving a client id and then to query a
stream's info, using `tokio`.

```rust
use twitch_helix::{Client, OAuthRequest, HelixRequest};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let oauth_token = "<insert your oauth token>";
	let mut client = Client::new(oauth_token);
	let validation = client
		.request_oauth(&twitch_helix::request::oauth::validate::Request)
		.await?
		.into_result()?;
	
	let channel_info_request = twitch_helix::request::search::channel::Request::new(channel);
	let channel_info_response = client
		.request_helix(&channel_info_request, &validation.client_id)
		.await?
		.into_result()?
		.data;
	let channel_info = channel_info_request.channel(channel_info_response)?;
}
```
