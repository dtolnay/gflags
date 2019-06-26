use proc_macro2::TokenStream;
use syn::{parse_quote, IntSuffix, Lit, Type};

pub fn infer_type(expr: &TokenStream) -> Option<Type> {
    let lit: Lit = syn::parse2(expr.clone()).ok()?;

    match lit {
        Lit::Str(_) => Some(parse_quote!(&str)),
        Lit::Int(int) => match int.suffix() {
            IntSuffix::I8 => Some(parse_quote!(i8)),
            IntSuffix::I16 => Some(parse_quote!(i16)),
            IntSuffix::I32 => Some(parse_quote!(i32)),
            IntSuffix::I64 => Some(parse_quote!(i64)),
            IntSuffix::I128 => Some(parse_quote!(i128)),
            IntSuffix::Isize => Some(parse_quote!(isize)),
            IntSuffix::U8 => Some(parse_quote!(u8)),
            IntSuffix::U16 => Some(parse_quote!(u16)),
            IntSuffix::U32 => Some(parse_quote!(u32)),
            IntSuffix::U64 => Some(parse_quote!(u64)),
            IntSuffix::U128 => Some(parse_quote!(u128)),
            IntSuffix::Usize => Some(parse_quote!(usize)),
            IntSuffix::None => None,
        },
        Lit::Bool(_) => Some(parse_quote!(bool)),
        _ => None,
    }
}
