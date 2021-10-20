use crate::traits::HTMLAnchor;

/// Structure representing a html comment.
pub struct Comment {
    content: String,
}

impl Comment {
    /// Instantiates a new Comment structure.
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl HTMLAnchor for Comment {
    fn generate<W: std::io::Write>(
        &self,
        w: &mut W,
        indent_level: usize,
    ) -> Result<(), std::io::Error> {
        writeln!(w, "{}<!-- {} -->", "\t".repeat(indent_level), self.content)
    }
}
