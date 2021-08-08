use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::content::Content;

/// The structure of the body of a response.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct Response {
	/// The content of the response. Implies a VLayout.
	pub content: Vec<Content>,
	/// Extra metadata included for use with new features.
	pub extra: HashMap<String, Value>,
}
impl Response {
	/// The MIME type of the response.
	pub const MIME_TYPE: &'static str = "application/cbor+hcpresponse";
}
