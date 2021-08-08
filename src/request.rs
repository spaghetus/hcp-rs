use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// The format used in the bodies of HCP requests.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct Request {
	features: Vec<String>,
	identities: HashMap<String, Uuid>,
	extra: HashMap<String, Value>,
}
impl Request {
	/// The MIME type of the request.
	pub const MIME_TYPE: &'static str = "application/cbor+hcprequest";
}
