use crate::dispatch::Parser;

// Not public API.
#[doc(hidden)]
pub struct Flag {
    pub doc: &'static [&'static str],
    pub short: Option<char>,
    pub name: &'static str,
    pub placeholder: Option<&'static str>,
    pub parser: &'static dyn Parser,
}

inventory::collect!(Flag);
