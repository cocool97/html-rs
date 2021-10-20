use std::io::{Error, Write};

use crate::{models::HTMLHeadAnchor, traits::HTMLAnchor};

/// Structure representing the HTML `head` anchor content.
pub struct HTMLHead {
    anchors: Vec<HTMLHeadAnchor>,
}

impl HTMLHead {
    /// Instantiates a new HTMLHead.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new anchor to this element.
    pub fn add_anchor(&mut self, anchor: HTMLHeadAnchor) {
        self.anchors.push(anchor);
    }
}

impl Default for HTMLHead {
    fn default() -> Self {
        Self {
            anchors: Vec::<HTMLHeadAnchor>::new(),
        }
    }
}

impl HTMLAnchor for HTMLHead {
    fn generate<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        writeln!(w, "<head>")?;

        for anchor in &self.anchors {
            anchor.generate(w, indent + 1)?;
        }

        writeln!(w, "</head>")?;

        Ok(())
    }
}
