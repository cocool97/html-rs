use crate::traits::HTMLAnchor;

/// Structure representing a `<title>` anchor.
pub struct Title {
    content: String,
}

impl Title {
    /// Instantiates a new Title structure.
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl HTMLAnchor for Title {
    fn generate<W: std::io::Write>(
        &self,
        w: &mut W,
        indent_level: usize,
    ) -> Result<(), std::io::Error> {
        writeln!(
            w,
            "{}<title>{}</title>",
            "\t".repeat(indent_level),
            self.content
        )
    }
}
