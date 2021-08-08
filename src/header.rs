use serde::{Deserialize, Serialize};

/// The structure of the HCP_INFO header.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct Header {
	/// The identity scopes the server wants to receive.
	pub scopes: Vec<String>,
	/// The features the server supports.
	pub features: Vec<String>,
	/// The features the server requires.
	pub required: Vec<String>,
}
