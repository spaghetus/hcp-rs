use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A content node in an HCF or HCP response.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone)]
pub enum Content {
	/// A block of text.
	Text(String),
	/// A horizontal layout. Shouldn't have any HLayout children.
	HLayout(Vec<Content>),
	/// A vertical layout. Shouldn't have any VLayout children. The root is an implicit VLayout.
	VLayout(Vec<Content>),
	/// An inline layout. Shouldn't have any layout children.
	InlineLayout(Vec<Content>),
	/// A table layout. All children should be HLayouts.
	Table(Vec<Content>),
	/// A live section that updates regularly.
	Live(
		/// The URL from which content should be fetched.
		String,
		/// The interval at which it should be fetched. Clients may disregard unreasonable values.
		f64,
	),
	/// A reference to non-text content. May be displayed as a download link or an embed, depending on client settings.
	Blob(
		/// The URL at which the blob content is located.
		String,
		/// The MIME type of the blob content.
		String,
		/// The text description of the blob content.
		String,
	),
	/// A list of choices that may be collapsed or differently arranged depending on client settings.
	/// Should have only Text, Blob, and Ref children.
	Menu(Vec<Content>),
	/// A reference to text content; preferably HCF or HCP-RESPONSE.
	Ref(
		/// The URL of the text content.
		String,
		/// The text description of the content.
		String,
	),
	/// An IF for templating. Clients should not render this and may display a warning if it falls through to them.
	If(String, Box<Content>, Box<Content>),
	/// An include for templating. Clients should not render this and may display a warning if it falls through to them.
	Include(String),
	/// A replace for templating. Clients should not render this and may display a warning if it falls through to them.
	Ctx(String),
	/// A form. Clients should display a warning and prevent submission if a form with a password uses an insecure URL, unless it resolves to the loopback.
	Form(
		/// The content of the form.
		Vec<Content>,
		/// The URL to which the form should be sent by POST in the format of an HTML form.
		String,
	),
	/// A field. Should be an ancestor of Form.
	Field(
		/// The name of the field.
		String,
		/// The label for the field.
		String,
		/// The type of the field. Assuming no extra features, it can be text, number, url, password, username, or a MIME type.
		String,
	),
}

impl Content {
	/// Perform templating.
	pub fn template(self, context: &HashMap<String, Content>, flags: &Vec<String>) -> Content {
		match self {
			Content::HLayout(content) => Content::HLayout(
				content
					.iter()
					.map(|v| v.clone().template(context, flags))
					.collect(),
			),
			Content::VLayout(content) => Content::VLayout(
				content
					.iter()
					.map(|v| v.clone().template(context, flags))
					.collect(),
			),
			Content::InlineLayout(content) => Content::InlineLayout(
				content
					.iter()
					.map(|v| v.clone().template(context, flags))
					.collect(),
			),
			Content::Table(content) => Content::Table(
				content
					.iter()
					.map(|v| v.clone().template(context, flags))
					.collect(),
			),
			Content::Menu(content) => Content::Menu(
				content
					.iter()
					.map(|v| v.clone().template(context, flags))
					.collect(),
			),
			Content::If(flag, yes, no) => {
				if flags.contains(&flag) {
					yes.template(context, flags)
				} else {
					no.template(context, flags)
				}
			}
			Content::Include(_) => todo!(),
			Content::Ctx(key) => context
				.get(&key)
				.map(|v| v.clone().template(context, flags))
				.unwrap_or(Content::Text("BAD CONTEXT".to_string())),
			Content::Form(content, u) => Content::Form(
				content
					.iter()
					.map(|v| v.clone().template(context, flags))
					.collect(),
				u,
			),
			_ => self,
		}
	}
}
