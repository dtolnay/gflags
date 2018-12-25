use proc_macro2::TokenStream;
use quote::quote;

use crate::{error, infer};
use crate::parse::Flag;

pub fn expand(input: Flag) -> TokenStream {
    let Flag {
        doc,
        vis,
        short,
        long,
        placeholder,
        ty,
        default,
    } = input;

    let short = match short {
        Some(short) => {
            let ch = short.ch();
            quote!(Some(#ch))
        }
        None => quote!(None),
    };

    let ident = long.to_ident();
    let name_str = long.to_string();
    let ty = match ty {
        Some(ty) => ty,
        None => match &default {
            Some(default) => match infer::infer_type(default) {
                Some(ty) => ty,
                None => {
                    let default = default.clone();
                    return error::cannot_infer(long, placeholder, Some(default));
                }
            },
            None => return error::cannot_infer(long, placeholder, None),
        },
    };

    let placeholder = match placeholder {
        Some(placeholder) => {
            if quote!(#ty).to_string() == "bool" {
                return error::bool_placeholder(placeholder);
            }
            let placeholder_str = placeholder.ident.to_string();
            quote!(Some(#placeholder_str))
        }
        None => quote!(None),
    };

    let init = match default {
        Some(default) => quote!(gflags::Flag::new(&(#default))),
        None => quote!(gflags::Flag::null()),
    };

    quote! {
        #vis static #ident: gflags::Flag<#ty> = #init;
        gflags::submit! {
            gflags::registry::Flag {
                doc: &[#(#doc),*],
                short: #short,
                name: #name_str,
                placeholder: #placeholder,
                parser: &#ident,
            }
        }
    }
}
