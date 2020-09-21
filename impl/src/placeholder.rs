use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, LitStr, Token};

pub struct Placeholder {
    langle: Token![<],
    pub text: PlaceholderToken,
    rangle: Token![>],
}

pub enum PlaceholderToken {
    Ident(Ident),
    Lit(LitStr),
}

impl Parse for Placeholder {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Placeholder {
            langle: input.parse()?,
            text: input.parse()?,
            rangle: input.parse()?,
        })
    }
}

impl Parse for PlaceholderToken {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            input.parse().map(PlaceholderToken::Lit)
        } else {
            input.call(Ident::parse_any).map(PlaceholderToken::Ident)
        }
    }
}

impl ToTokens for Placeholder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.langle.to_tokens(tokens);
        self.text.to_tokens(tokens);
        self.rangle.to_tokens(tokens);
    }
}

impl ToTokens for PlaceholderToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PlaceholderToken::Ident(ident) => ident.to_tokens(tokens),
            PlaceholderToken::Lit(lit) => lit.to_tokens(tokens),
        }
    }
}
