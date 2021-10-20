use crate::{traits::HTMLAnchor, utils};

/// Structure representing a `<meta>` anchor.
pub struct Meta {
    charset: Option<String>,
}

impl Meta {
    /// Instantiates a new Meta structure.
    pub fn new<S: Into<String>>(charset: Option<S>) -> Self {
        Self {
            charset: charset.map(S::into),
        }
    }
}

impl HTMLAnchor for Meta {
    fn generate<W: std::io::Write>(
        &self,
        w: &mut W,
        indent_level: usize,
    ) -> Result<(), std::io::Error> {
        writeln!(
            w,
            "{}<meta{}>",
            "\t".repeat(indent_level),
            utils::generate_attribute("charset", &self.charset)
        )
    }
}
