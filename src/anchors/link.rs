use crate::{traits::HTMLAnchor, utils};

/// Structure representing a `<link>` anchor.
pub struct Link {
    rel: Option<String>,
    stylesheet: Option<String>,
}

impl Link {
    /// Instantiates a new Link structure.
    pub fn new<S: Into<String>>(rel: Option<S>, stylesheet: Option<S>) -> Self {
        Self {
            rel: rel.map(S::into),
            stylesheet: stylesheet.map(S::into),
        }
    }
}

impl HTMLAnchor for Link {
    fn generate<W: std::io::Write>(
        &self,
        w: &mut W,
        indent_level: usize,
    ) -> Result<(), std::io::Error> {
        writeln!(
            w,
            "{}<link{}{}>",
            "\t".repeat(indent_level),
            utils::generate_attribute("rel", &self.rel),
            utils::generate_attribute("stylesheet", &self.stylesheet)
        )
    }
}
