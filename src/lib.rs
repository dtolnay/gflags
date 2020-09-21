//! [![github]](https://github.com/dtolnay/gflags)&ensp;[![crates-io]](https://crates.io/crates/gflags)&ensp;[![docs-rs]](https://docs.rs/gflags)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Command line flag library in the style of [gflags (formerly Google
//! Commandline Flags)][gflags].
//!
//! [gflags]: https://gflags.github.io/gflags/
//!
//! Quoting directly from the C++ gflags documentation, because the concept is
//! the same here:
//!
//! <br>
//!
//! > ---
//! >
//! > **Commandline flags** are flags that users specify on the command line
//! > when they run an executable. In the command
//! >
//! > ```text
//! > fgrep -l -f /var/tmp/foo johannes brahms
//! > ```
//! >
//! > `-l` and `-f /var/tmp/foo` are the two commandline flags. (`johannes` and
//! > `brahms`, which don't start with a dash, are **commandline arguments**.)
//! >
//! > Typically, an application lists what flags the user is allowed to pass in,
//! > and what arguments they take -- in this example, `-l` takes no argument,
//! > and `-f` takes a string (in particular, a filename) as an argument. Users
//! > can use a library to help parse the commandline and store the flags in
//! > some data structure.
//! >
//! > Gflags, the commandline flags library used within Google, differs from
//! > other libraries, such as `getopt()`, in that flag definitions can be
//! > scattered around the source code, and not just listed in one place such as
//! > `main()`. In practice, this means that a single source-code file will
//! > define and use flags that are meaningful to that file. Any application
//! > that links in that file will get the flags, and the gflags library will
//! > automatically handle that flag appropriately.
//! >
//! > There's significant gain in flexibility, and ease of code reuse, due to
//! > this technique.
//! >
//! > ---
//!
//! <br>
//!
//! This style of flag registration is better suited for large scale development
//! than maintaining a single central list of flags, as the central list would
//! become an endless source of merge conflicts in an application developed
//! simultaneously by hundreds of developers.
//!
//! # Defining flags
//!
//! Flags may be defined from any source file through the [`gflags::define!`]
//! macro. There is no central list of all the flags of the application. (That's
//! the point and advantage of gflags for large-scale development compared to
//! other flags libraries.)
//!
//! [`gflags::define!`]: macro.define.html
//!
//! ```
//! gflags::define! {
//!     /// Include 'advanced' options in the menu listing.
//!     --big_menu = true
//! }
//!
//! gflags::define! {
//!     /// Comma-separated list of languages to offer in the 'lang' menu.
//!     -l, --language <LANG> = "english,french,german"
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Flags are required to have a long name (like `--verbose`) and may optionally
//! have a short name (like `-v`). Flags must have exactly one long name and at
//! most one short name; multiple different aliases for the same flag is not
//! supported.
//!
//! Flags of a type other than bool may have an optional value-placeholder like
//! `<LANG>`. This is optional and purely cosmetic. It appears in help text.
//!
//! # Accessing flags
//!
//! Somewhere early in your application, call [`gflags::parse()`] to parse the
//! command line. This call returns a `Vec<&str>` containing everything on the
//! command line which is not a flag (these are sometimes known as positional
//! arguments) in a vector.
//!
//! [`gflags::parse()`]: fn.parse.html
//!
//! After `gflags::parse()` has been called, the value of each flag is available
//! in the `.flag` field of the flag's long name.
//!
//! ```
//! gflags::define! {
//!     --print-args = false
//! }
//!
//! fn main() {
//!     let args = gflags::parse();
//!
//!     if PRINT_ARGS.flag {
//!         println!("args = {:?}", args);
//!     }
//! }
//! ```
//!
//! As shown in this snippet, flag names may contain hyphens, in which case the
//! variable through which the flag's value can be accessed has underscores in
//! place of the hyphens.
//!
//! Additionally every flag provides a method `.is_present()` to query whether
//! that flag was provided on the command line. When using flags for which a
//! default value is not provided, be sure to check `.is_present()` because
//! accessing `.flag` when not present will cause a panic. Note also that flags
//! without a default value must specify their data type, as below.
//!
//! ```
//! use std::path::Path;
//!
//! gflags::define! {
//!     /// Search for patterns from the given file, with one pattern per line.
//!     -f, --file: &Path
//! }
//!
//! fn main() {
//!     let patterns = gflags::parse();
//!
//!     if FILE.is_present() {
//!         let path = FILE.flag;
//!         println!("searching for patterns from file: {}", path.display());
//!     } else {
//!         println!("searching for patterns given on command line: {:?}", patterns);
//!     }
//! }
//! ```
//!
//! # Printing help
//!
//! There is no built-in `-h` flag for help, but you can define your own and
//! call [`gflags::print_help_and_exit()`] to render the documentation of all
//! flags.
//!
//! [`gflags::print_help_and_exit()`]: fn.print_help_and_exit.html
//!
//! ```
//! gflags::define! {
//!     -h, --help = false
//! }
//!
//! fn main() {
//!     gflags::parse();
//!     if HELP.flag {
//!         gflags::print_help_and_exit(0);
//!     }
//!
//!     /* ... */
//! }
//! ```
//!
//! For some of the flag definitions shown in this documentation, the help text
//! would be rendered as follows.
//!
//! ```text
//!         --big_menu
//!             Include 'advanced' options in the menu listing.
//!
//!     -f, --file
//!             Search for patterns from the given file, with one pattern per line.
//!
//!     -l, --language <LANG>
//!             Comma-separated list of languages to offer in the 'lang' menu.
//! ```
//!
//! The flags are listed in alphabetical order by long name.
//!
//! You will likely want to print your own content above this including the
//! application name, version, author, introductory explanation, and usage
//! strings.
//!
//! # Custom data types
//!
//! The `gflags::define!` macro is extensible to custom data types by providing
//! an impl of [`gflags::custom::Value`] for your type.
//!
//! [`gflags::custom::Value`]: custom/trait.Value.html
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
//!         match arg.get_str() {
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

#![doc(html_root_url = "https://docs.rs/gflags/0.3.6")]
#![allow(clippy::needless_doctest_main)]

macro_rules! eprintln {
    ($($tt:tt)*) => {{
        use std::io::Write;
        let _ = std::writeln!(std::io::stderr(), $($tt)*);
    }};
}

mod arg;
mod atomic;
mod dispatch;
mod error;
mod help;
mod name;
mod parse;
mod state;
mod token;
mod value;

pub mod custom;

pub use crate::help::print_help_and_exit;
pub use crate::parse::{parse, parse_os};
pub use crate::state::Flag;

// Not public API.
#[doc(hidden)]
pub mod registry;

#[doc(hidden)]
pub use inventory;

#[doc(hidden)]
pub use gflags_impl as r#impl;

/// Entry point for registering a flag from any source file.
///
/// # Examples
///
/// Please refer to the [crate level documentation](index.html) for several
/// usage examples.
///
/// # Grammar
///
/// The complete input grammar is as follows.
///
/// - Zero or more doc comments: `/// ...`. These are rendered into the
///   generated help text.
///
/// - Optional visibility specifier like `pub` or `pub(crate)`. This controls
///   the scope of code that is allowed to see the value of this flag. By
///   default flags have private visibility, which is the default in Rust.
///
/// - Optional short name for the flag, like `-v`, followed by a comma.
///
/// - Long name for the flag, like `--verbose`. Long name is mandatory.
///
/// - Optional value-placeholder in angle brackets, like `<FILE>`. This is
///   cosmetic and appears in generated help text.
///
/// - Optional value type preceded by colon, like `: &str`. Type is required if
///   there is no default value or the default value is not a Rust string or
///   boolean or integer literal.
///
/// - Optional default value preceded by equal-sign: `= "default"`.
///
/// Invocation containing as few of the above as possible:
///
/// ```
/// gflags::define! {
///     --minimal1: bool
/// }
/// #
/// # fn main() {}
/// ```
///
/// Another way to show as few as possible. Either type or default value must be
/// specified.
///
/// ```
/// gflags::define! {
///     --minimal2 = "default value"
/// }
/// #
/// # fn main() {}
/// ```
///
/// Showing everything at once:
///
/// ```
/// # mod path {
/// #     pub mod to {
/// #         pub const DEFAULT: u32 = 0;
/// #     }
/// # }
/// #
/// gflags::define! {
///     /// Documentation!
///     pub -m, --maximal <VALUE>: u32 = path::to::DEFAULT
/// }
/// #
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! define {
    ($($flag:tt)*) => {
        gflags::r#impl::define_impl! {
            $($flag)*
        }
    };
}
