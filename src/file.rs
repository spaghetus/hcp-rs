use std::collections::HashMap;

use crate::content::Content;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The structure of an HCF.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct File {
	/// The content of the HCF. Implies a VLayout.
	pub content: Vec<Content>,
	/// Extra data for use with features.
	pub extra: HashMap<String, Value>,
}
impl File {
	/// The MIME type of a YAML HCF.
	pub const YAML_MIME_TYPE: &'static str = "application/yaml+hcf";
	/// The MIME type of a JSON HCF.
	pub const JSON_MIME_TYPE: &'static str = "application/json+hcf";
}
impl Default for File {
	fn default() -> Self {
		File {
			content: vec![],
			extra: HashMap::new(),
		}
	}
}
