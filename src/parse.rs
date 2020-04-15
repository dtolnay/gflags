use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::process;

use crate::name::Name;
use crate::registry::Flag;
use crate::token::{Token, Tokenizer};

/// Initialize the value of all flags based on arguments from the command line
/// at runtime.
///
/// This function must be called before accessing the values of any flags. After
/// this function has been called, the values of flags are available in the
/// `.flag` field of each flag.
///
/// The return vector contains everything on the command line which is not a
/// flag. These are sometimes called positional arguments.
///
/// # Examples
///
/// If we execute the following program with command line `./my_program a b c`
/// then nothing is printed because `--print-args` is not set. If we execute it
/// with `./my_program --print-args a b c` then the positional arguments `a` `b`
/// `c` are printed.
///
/// ```
/// gflags::define! {
///     --print-args = false
/// }
///
/// fn main() {
///     let args = gflags::parse();
///
///     if PRINT_ARGS.flag {
///         println!("args = {:?}", args);
///     }
/// }
/// ```
///
/// # Aborts
///
/// Aborts the process with an error message if the command line does not
/// conform to the flags defined by the application, or if any of the positional
/// arguments are non-UTF8. Use [`gflags::parse_os`] if you need to support
/// non-UTF8 positional arguments.
///
/// [`gflags::parse_os`]: crate::parse_os()
pub fn parse() -> Vec<&'static str> {
    fn to_str_or_abort(os_str: &OsStr) -> &str {
        os_str.to_str().unwrap_or_else(|| {
            eprintln!("Unsupported non-UTF8 command line argument");
            process::exit(1);
        })
    }

    parse_os().into_iter().map(to_str_or_abort).collect()
}

/// Initialize the value of all flags, accepting non-UTF8 positional arguments.
///
/// Equivalent to [`gflags::parse`] in all ways except that non-UTF8 positional
/// arguments are not an error. Note that non-UTF8 *flag values* are allowed
/// even by `gflags::parse`.
///
/// [`gflags::parse`]: crate::parse()
pub fn parse_os() -> Vec<&'static OsStr> {
    let mut shorts = BTreeMap::new();
    let mut longs = BTreeMap::new();
    for flag in inventory::iter::<Flag> {
        if let Some(short) = flag.short {
            shorts.insert(short, flag);
        }
        longs.insert(flag.name, flag);
    }

    let mut args = Vec::new();
    let mut tokens = Tokenizer::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Short(ch) => match shorts.get(&ch) {
                Some(flag) => {
                    let name = Name::short(ch);
                    flag.parser.parse(name, &mut tokens);
                }
                None => {
                    eprintln!("Unrecognized flag: -{}", ch);
                    process::exit(1);
                }
            },
            Token::Long(name) => match longs.get(name) {
                Some(flag) => {
                    let name = Name::long(flag.name);
                    flag.parser.parse(name, &mut tokens);
                }
                None => {
                    if name.starts_with("no") {
                        if let Some(flag) = longs.get(&name[2..]) {
                            if flag.parser.is_bool() {
                                flag.parser.unset_bool();
                                continue;
                            }
                        }
                    }
                    eprintln!("Unrecognized flag: --{}", name);
                    process::exit(1);
                }
            },
            Token::LongEq(name, arg) => {
                if let Some(flag) = longs.get(name) {
                    if flag.parser.is_bool() {
                        eprintln!("Unexpected argument {:?} for flag: --{}={}", arg, name, arg);
                        process::exit(1);
                    }

                    let name = Name::long(flag.name);

                    // Prepare an iterator and tokenizer for just this arg, then
                    // parse the arg for the flag
                    let arg = vec![OsStr::new(arg)];
                    let mut tokens = Tokenizer::iterate(arg);

                    flag.parser.parse(name, &mut tokens);
                } else {
                    eprintln!("Unrecognized flag: --{}", name);
                    process::exit(1);
                }
            }
            Token::Arg(arg) => args.push(arg),
        }
    }

    args
}
