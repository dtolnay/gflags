use crate::name::{Long, Name, Short};
use crate::placeholder::Placeholder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{bracketed, LitStr, Token, Type, Visibility};

pub struct Flag {
    pub doc: Vec<String>,
    pub vis: Visibility,
    pub short: Option<Short>,
    pub long: Long,
    pub placeholder: Option<Placeholder>,
    pub ty: Option<Type>,
    pub default: Option<TokenStream>,
}

mod keyword {
    syn::custom_keyword!(doc);
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut doc = Vec::new();

        while input.parse::<Option<Token![#]>>()?.is_some() {
            let content;
            bracketed!(content in input);
            content.parse::<keyword::doc>()?;
            content.parse::<Token![=]>()?;
            let lit: LitStr = content.parse()?;
            let mut string = lit.value();
            if string.starts_with(' ') {
                string.remove(0);
            }
            doc.push(string);
        }

        let vis: Visibility = input.parse()?;

        if !input.peek(Token![-]) {
            return Err(input.error("expected flag"));
        }

        let mut short = None::<Short>;
        let mut long = None::<Long>;
        while input.peek(Token![-]) {
            match input.parse()? {
                Name::Short(Short {
                    hyphen,
                    ch: short_ident,
                }) => {
                    if short.is_some() {
                        let spanned = quote!(#hyphen #short_ident);
                        let msg = "unsupported second short flag";
                        return Err(Error::new_spanned(spanned, msg));
                    }
                    short = Some(Short {
                        hyphen,
                        ch: short_ident,
                    });
                }
                Name::Long(Long {
                    hyphen1,
                    hyphen2,
                    segments: long_ident,
                }) => {
                    if long.is_some() {
                        let spanned = quote!(#hyphen1 #hyphen2 #long_ident);
                        let msg = "unsupported second long flag";
                        return Err(Error::new_spanned(spanned, msg));
                    }
                    long = Some(Long {
                        hyphen1,
                        hyphen2,
                        segments: long_ident,
                    });
                }
            }
            if input.parse::<Option<Token![,]>>()?.is_none() {
                break;
            }
        }

        let long = match long {
            Some(name) => name,
            None => {
                return Err(Error::new_spanned(short, "missing long flag"));
            }
        };

        let placeholder = if input.peek(Token![<]) {
            input.parse().map(Some)?
        } else {
            None
        };

        let ty = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let ty: Type = input.parse()?;
            Some(ty)
        } else {
            None
        };

        let default = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let default: TokenStream = input.parse()?;
            Some(default)
        } else {
            None
        };

        Ok(Flag {
            doc,
            vis,
            short,
            long,
            placeholder,
            ty,
            default,
        })
    }
}
