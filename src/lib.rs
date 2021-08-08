#![forbid(missing_docs)]

//! A reference implementation and working definition of the HCP.

mod content;
mod file;
mod header;
mod request;
mod response;
pub use content::Content;
pub use file::File;
pub use header::Header;
pub use request::Request;
pub use response::Response;
