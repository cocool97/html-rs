#![crate_type = "lib"]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

//! `html-rs` is a rust crate providing an easy way to create, generate and write HTML5 documents.

/// Module containing all anchor definitions.
pub mod anchors;

mod body;
mod doctype;
mod dom;
mod head;
mod models;
mod traits;
mod utils;

pub use body::HTMLBody;
pub use doctype::HTMLDoctype;
pub use dom::HTMLDOM;
pub use head::HTMLHead;
pub use models::{HTMLAnchor, HTMLBodyAnchor, HTMLHeadAnchor};
