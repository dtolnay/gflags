//! Interface for defining your own flag data type.
//!
//! A list of the built-in supported flag types may be found in the list of
//! impls of the [`Value`] trait. The gflags library is extensible to custom
//! data types by providing your own impl of that trait.
//!
//! [`Value`]: trait.Value.html
//!
//! # Examples
//!
//! ```
//! use gflags::custom::{Arg, Error, Result, Value};
//!
//! gflags::define! {
//!     --color <WHEN>: Color = Color::Auto
//! }
//!
//! enum Color {
//!     Never,
//!     Always,
//!     Auto,
//! }
//!
//! impl Value for Color {
//!     fn parse(arg: Arg) -> Result<Self> {
//!         match arg.get_string().as_str() {
//!             "never" => Ok(Color::Never),
//!             "always" => Ok(Color::Always),
//!             "auto" => Ok(Color::Auto),
//!             _ => Err(Error::new("invalid color")),
//!         }
//!     }
//! }
//! #
//! # fn main() {}
//! ```

pub use crate::arg::Arg;
pub use crate::error::{Error, Result};
pub use crate::value::Value;
