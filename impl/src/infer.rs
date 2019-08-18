use proc_macro2::TokenStream;
use syn::{parse_quote, Lit, Type};

pub fn infer_type(expr: &TokenStream) -> Option<Type> {
    let lit: Lit = syn::parse2(expr.clone()).ok()?;

    match lit {
        Lit::Str(_) => Some(parse_quote!(&str)),
        Lit::Int(int) => match int.suffix() {
            "i8" => Some(parse_quote!(i8)),
            "i16" => Some(parse_quote!(i16)),
            "i32" => Some(parse_quote!(i32)),
            "i64" => Some(parse_quote!(i64)),
            "i128" => Some(parse_quote!(i128)),
            "isize" => Some(parse_quote!(isize)),
            "u8" => Some(parse_quote!(u8)),
            "u16" => Some(parse_quote!(u16)),
            "u32" => Some(parse_quote!(u32)),
            "u64" => Some(parse_quote!(u64)),
            "u128" => Some(parse_quote!(u128)),
            "usize" => Some(parse_quote!(usize)),
            _ => None,
        },
        Lit::Bool(_) => Some(parse_quote!(bool)),
        _ => None,
    }
}
