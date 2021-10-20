use crate::{traits::HTMLAnchor, HTMLBody, HTMLDoctype, HTMLHead};

use std::io::{Error, Write};

/// Describes the root HTML DOM Object.
pub struct HTMLDOM {
    /// The HTML Doctype anchor.
    doctype: HTMLDoctype,
    /// The HTML DOM language.
    lang: Option<String>,
    /// The HTML <head> anchor content.
    head: HTMLHead,
    /// The HTML <body> anchor content.
    body: HTMLBody,
}

impl HTMLDOM {
    /// Instantiates a new HTML root DOM object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the HTMLDoctype
    fn doctype(&self) -> &HTMLDoctype {
        &self.doctype
    }

    /// Gets the DOM language
    fn lang(&self) -> Option<&String> {
        self.lang.as_ref()
    }

    /// Gets the HTMLHead
    fn head(&self) -> &HTMLHead {
        &self.head
    }

    /// Gets the HTMLBody
    fn body(&self) -> &HTMLBody {
        &self.body
    }

    /// Sets the HTML DOM `lang` attribute
    pub fn set_lang<S: Into<String>>(&mut self, lang: S) {
        self.lang = Some(lang.into());
    }

    /// Sets the HTML head anchor content
    pub fn set_head(&mut self, head: HTMLHead) {
        self.head = head;
    }

    /// Builds and consumes the HTML DOM.
    pub fn build_dom<Wr: Write>(self, mut writer: Wr) -> Result<(), Error> {
        self.generate(&mut writer, 0)
    }
}

impl Default for HTMLDOM {
    fn default() -> Self {
        Self {
            doctype: Default::default(),
            lang: None,
            head: Default::default(),
            body: Default::default(),
        }
    }
}

impl HTMLAnchor for HTMLDOM {
    fn generate<Wr: Write>(&self, w: &mut Wr, indent: usize) -> Result<(), Error> {
        if let Err(e) = self.doctype().generate(w, indent) {
            return Err(e);
        }

        if let Some(lang) = self.lang() {
            writeln!(w, "<html lang=\"{}\">", lang)?;
        } else {
            writeln!(w, "<html>")?;
        }

        if let Err(e) = self.head().generate(w, indent) {
            return Err(e);
        }

        if let Err(e) = self.body().generate(w, indent) {
            return Err(e);
        }

        writeln!(w, "</html>")?;

        Ok(())
    }
}
