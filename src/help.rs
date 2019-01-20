use std::io::{self, Write};
use std::process;

use crate::registry::Flag;

/// Print the names and descriptions of all the flags.
///
/// There is no built-in `-h` flag in gflags for help. If you want one, go ahead
/// and define your own and call this function to render the documentation of
/// all flags. Output goes to stdout if the exit code is zero, and stderr if
/// nonzero.
///
/// # Example
///
/// ```
/// gflags::define! {
///     -h, --help = false
/// }
///
/// fn main() {
///     gflags::parse();
///     if HELP.flag {
///         gflags::print_help_and_exit(0);
///     }
///
///     /* ... */
/// }
/// ```
///
/// # Output
///
/// For some of the flag definitions shown in [the crate level
/// documentation](index.html), the help text would be rendered as follows.
///
/// ```text
///         --big_menu
///             Include 'advanced' options in the menu listing.
///
///     -f, --file
///             Search for patterns from the given file, with one pattern per line.
///
///     -l, --language <LANG>
///             Comma-separated list of languages to offer in the 'lang' menu.
/// ```
///
/// The flags are listed in alphabetical order by long name.
///
/// **Tip:** You will likely want to print your own content above this including
/// the application name, version, author, introductory explanation, and usage
/// strings.
pub fn print_help_and_exit(code: i32) -> ! {
    if code == 0 {
        let _ = try_print_help(&mut io::stdout().lock());
    } else {
        let _ = try_print_help(&mut io::stderr().lock());
    };

    process::exit(code);
}

fn try_print_help(stream: &mut Write) -> io::Result<()> {
    let mut flags = inventory::iter::<Flag>.into_iter().collect::<Vec<_>>();
    flags.sort_by_key(|flag| flag.name);

    let has_short = flags.iter().any(|flag| flag.short.is_some());

    for flag in flags {
        write!(stream, "    ")?;
        if has_short {
            match flag.short {
                Some(short) => write!(stream, "-{}, ", short)?,
                None => write!(stream, "    ")?,
            }
        }
        write!(stream, "--{}", flag.name)?;
        if let Some(placeholder) = flag.placeholder {
            write!(stream, " <{}>", placeholder)?;
        }
        writeln!(stream)?;
        for line in flag.doc {
            let line = line.trim_end();
            if line.is_empty() {
                writeln!(stream)?;
            } else {
                writeln!(stream, "            {}", line)?;
            }
        }
        writeln!(stream)?;
    }

    Ok(())
}
