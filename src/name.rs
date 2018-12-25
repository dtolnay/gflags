use std::fmt::{self, Debug, Display};

#[derive(Copy, Clone)]
pub struct Name {
    style: Style,
}

#[derive(Copy, Clone)]
enum Style {
    Short(char),
    Long(&'static str),
}

impl Name {
    pub(crate) fn short(ch: char) -> Self {
        Name {
            style: Style::Short(ch),
        }
    }

    pub(crate) fn long(name: &'static str) -> Self {
        Name {
            style: Style::Long(name),
        }
    }
}

impl Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.style {
            Style::Short(ch) => write!(formatter, "-{}", ch),
            Style::Long(name) => write!(formatter, "--{}", name),
        }
    }
}

impl Debug for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.style {
            Style::Short(ch) => formatter.debug_tuple("Short").field(&ch).finish(),
            Style::Long(name) => formatter.debug_tuple("Long").field(&name).finish(),
        }
    }
}
