// Imports
use crate::helix_url;

#[test]
fn helix_url() {
	let url = helix_url!(example / request);
	assert_eq!(url.as_str(), "https://api.twitch.tv/helix/example/request");
	assert_eq!(url.host_str(), Some("api.twitch.tv"));
}
