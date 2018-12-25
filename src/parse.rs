use std::collections::BTreeMap;
use std::ffi::OsString;
use std::process;

use crate::name::Name;
use crate::registry::Flag;
use crate::token::{Token, Tokenizer};

/// Initialize the value of all flags based on arguments from the command line
/// at runtime.
///
/// This function must be called before accessing the values of any flags. After
/// this function has been called, the values of flags are available in the
/// `.FLAG` field of each flag.
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
///     if print_args.FLAG {
///         println!("args = {:?}", args);
///     }
/// }
/// ```
pub fn parse() -> Vec<OsString> {
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
            Token::Long(name) => match longs.get(name.as_str()) {
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
            Token::Arg(arg) => args.push(arg),
        }
    }

    args
}
