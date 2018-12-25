use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};
use syn::ext::IdentExt;

pub struct Placeholder {
    langle: Token![<],
    pub ident: Ident,
    rangle: Token![>],
}

impl Parse for Placeholder {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Placeholder {
            langle: input.parse()?,
            ident: input.call(Ident::parse_any)?,
            rangle: input.parse()?,
        })
    }
}

impl ToTokens for Placeholder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.langle.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.rangle.to_tokens(tokens);
    }
}
