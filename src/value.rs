use crate::custom::Arg;
use crate::error::{Error, Result};
use crate::state::Flag;
use std::ffi::OsStr;
use std::path::Path;

/// Types that may be the data type of a flag.
///
/// The gflags library provides implementations of `Value` for several primitive
/// types like `&str` and `u64`. Refer to the [module documentation](index.html)
/// for an example of implementing `Value` for your own types.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub trait Value: Sized + 'static {
    fn parse(arg: Arg) -> Result<Self>;

    // Not public API.
    #[doc(hidden)]
    const IS_BOOL: bool = false;

    // Not public API.
    #[doc(hidden)]
    fn set_bool(_flag: &Flag<Self>, _value: &'static bool) {
        panic!("not bool");
    }
}

impl Value for bool {
    fn parse(_arg: Arg) -> Result<Self> {
        panic!("bool flag does not accept argument");
    }

    const IS_BOOL: bool = true;

    fn set_bool(flag: &Flag<Self>, value: &'static bool) {
        flag.set_bool(value);
    }
}

impl Value for &'static str {
    fn parse(arg: Arg) -> Result<Self> {
        Ok(arg.get_str())
    }
}

impl Value for &'static OsStr {
    fn parse(arg: Arg) -> Result<Self> {
        Ok(arg.get_raw())
    }
}

impl Value for &'static Path {
    fn parse(arg: Arg) -> Result<Self> {
        <&OsStr>::parse(arg).map(Path::new)
    }
}

macro_rules! impl_value_for_primitive {
    ($($primitive:ident)*) => {
        $(
            impl Value for $primitive {
                fn parse(arg: Arg) -> Result<Self> {
                    arg.get_str().parse().map_err(Error)
                }
            }
        )*
    };
}

impl_value_for_primitive!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
