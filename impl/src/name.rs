use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::{self, Display};
use syn::ext::IdentExt;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::{Pair, Punctuated};
use syn::{Ident, Token};

pub enum Name {
    Short(Short),
    Long(Long),
}

pub struct Short {
    pub hyphen: Token![-],
    pub ch: Ident,
}

pub struct Long {
    pub hyphen1: Token![-],
    pub hyphen2: Token![-],
    pub segments: Punctuated<Ident, Token![-]>,
}

impl Short {
    pub fn ch(&self) -> char {
        self.ch.to_string().chars().next().unwrap()
    }
}

impl Long {
    pub fn to_ident(&self) -> Ident {
        let span = self.segments[0].span();
        let symbol = self.to_string().replace('-', "_").to_uppercase();
        Ident::new(&symbol, span)
    }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> Result<Self> {
        let hyphen1: Token![-] = input.parse()?;

        if input.fork().call(Ident::parse_any).is_ok() {
            let short = input.call(Ident::parse_any)?;
            if short.to_string().chars().count() > 1 {
                let spanned = quote!(#hyphen1 #short);
                let msg = "short flag must be a single character";
                return Err(Error::new_spanned(spanned, msg));
            }
            Ok(Name::Short(Short {
                hyphen: hyphen1,
                ch: short,
            }))
        } else if let Some(hyphen2) = input.parse()? {
            let segments = input.call(parse_long_segments)?;
            Ok(Name::Long(Long {
                hyphen1,
                hyphen2,
                segments,
            }))
        } else {
            Err(input.error("expected short (-v) or long (--verbose) flag"))
        }
    }
}

fn parse_long_segments(input: ParseStream) -> Result<Punctuated<Ident, Token![-]>> {
    let mut segments = Punctuated::new();
    let first = input.call(Ident::parse_any)?;
    segments.push_value(first);

    while input.peek(Token![-]) {
        let hyphen: Token![-] = input.parse()?;
        segments.push_punct(hyphen);
        let segment = input.call(Ident::parse_any)?;
        segments.push_value(segment);
    }

    Ok(segments)
}

impl ToTokens for Short {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.hyphen.to_tokens(tokens);
        self.ch.to_tokens(tokens);
    }
}

impl ToTokens for Long {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.hyphen1.to_tokens(tokens);
        self.hyphen2.to_tokens(tokens);
        self.segments.to_tokens(tokens);
    }
}

impl Display for Long {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for pair in self.segments.pairs() {
            match pair {
                Pair::Punctuated(ident, _hyphen) => write!(formatter, "{}-", ident)?,
                Pair::End(ident) => write!(formatter, "{}", ident)?,
            }
        }
        Ok(())
    }
}
