use prost::Message;
use std::fmt::{Debug, Error, Formatter};

pub struct Debuggable<'a>(Box<&'a dyn Message>);

impl<'a> Debuggable<'a> {
    pub fn from(message: &'a dyn Message) -> Self {
        Debuggable(Box::from(message))
    }
}

impl Debug for Debuggable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}
