use crate::traits::HTMLAnchor;

pub use std::io::{Error, Write};

/// Structure representing the HTML `doctype` anchor content.
pub struct HTMLDoctype {}

impl Default for HTMLDoctype {
    fn default() -> Self {
        Self {}
    }
}

impl HTMLAnchor for HTMLDoctype {
    fn generate<W: Write>(&self, w: &mut W, _indent: usize) -> Result<(), Error> {
        writeln!(w, "<!doctype html>")
    }
}
