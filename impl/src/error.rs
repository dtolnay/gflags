use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;

use crate::name::Long;
use crate::placeholder::Placeholder;

pub fn cannot_infer(
    long: Long,
    placeholder: Option<Placeholder>,
    default: Option<TokenStream>,
) -> TokenStream {
    let placeholder_elem = match &placeholder {
        Some(placeholder) => format!(" <{}>", placeholder.text.to_token_stream()),
        None => String::new(),
    };
    let default_elem = match default {
        Some(_) => " = ...",
        None => "",
    };
    let msg = format!(
        "unable to infer type; specify a type explicitly:\
         \n    --{}{}: &str{}\n",
        long, placeholder_elem, default_elem
    );
    let err = if let Some(span) = default {
        Error::new_spanned(span, &msg)
    } else if let Some(span) = placeholder {
        Error::new_spanned(span, &msg)
    } else {
        Error::new_spanned(long, &msg)
    };
    err.to_compile_error()
}

pub fn bool_placeholder(placeholder: Placeholder) -> TokenStream {
    let msg = "boolean flags are not allowed a placeholder";
    let err = Error::new_spanned(placeholder, msg);
    err.to_compile_error()
}
