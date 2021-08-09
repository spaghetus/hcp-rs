use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// The format used in the bodies of HCP requests.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct Request {
	/// The features the client would like to be taken into account for this request.
	pub features: Vec<String>,
	/// The identities the client is providing for this request.
	pub identities: HashMap<String, Uuid>,
	/// Extra data for features.
	pub extra: HashMap<String, Value>,
}
impl Request {
	/// The MIME type of the request.
	pub const MIME_TYPE: &'static str = "application/cbor+hcprequest";
}
impl Default for Request {
	fn default() -> Self {
		Request {
			features: vec![],
			identities: HashMap::new(),
			extra: HashMap::new(),
		}
	}
}
