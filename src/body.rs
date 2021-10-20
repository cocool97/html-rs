use std::io::{Error, Write};

use crate::traits::HTMLAnchor;

/// Structure representing the HTML `body` anchor content.
pub struct HTMLBody {}

impl Default for HTMLBody {
    fn default() -> Self {
        Self {}
    }
}

impl HTMLAnchor for HTMLBody {
    fn generate<W: Write>(&self, w: &mut W, _indent: usize) -> Result<(), Error> {
        writeln!(w, "<body>")?;

        writeln!(w, "</body>")?;

        Ok(())
    }
}
