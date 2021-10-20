use std::io::{Error, Write};

pub trait HTMLAnchor {
    fn generate<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error>;
}
