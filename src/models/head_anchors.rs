use crate::{
    anchors::{Comment, Link, Meta, Title},
    traits::HTMLAnchor,
};

use std::io::{Error, Write};

/// Enum representing all available `<head>` HTML anchors.
pub enum HTMLHeadAnchor {
    /// Value representing a <meta> anchor.
    Meta(Meta),
    /// Value representing a <title> anchor.
    Title(Title),
    /// Value representing a <link> anchor.
    Link(Link),
    /// Value representing a comment.
    Comment(Comment),
}

impl HTMLAnchor for HTMLHeadAnchor {
    fn generate<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        // Only forwards generate methods to underlying items
        match self {
            HTMLHeadAnchor::Meta(m) => m.generate(w, indent),
            HTMLHeadAnchor::Title(t) => t.generate(w, indent),
            HTMLHeadAnchor::Link(l) => l.generate(w, indent),
            HTMLHeadAnchor::Comment(c) => c.generate(w, indent),
        }
    }
}
