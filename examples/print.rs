use gflags::custom::{Arg, Error, Result, Value};

use std::path::Path;

const ABOUT: &str = "
Gflags is a library for distributed registration of command line flags in a
large application. This example program accepts various flags shown in the
gflags readme. The alphabetical listing of flags under `OPTIONS` is rendered
by gflags.

Project home page: https://github.com/dtolnay/gflags

USAGE:
    cargo run --example print -- --help
    cargo run --example print -- --language english
    cargo run --example print -- -l english
    cargo run --example print -- -l english --color always

OPTIONS:";

gflags::define! {
    /// Include 'advanced' options in the menu listing.
    --big_menu = true
}

gflags::define! {
    /// Comma-separated list of languages to offer in the 'lang' menu.
    -l, --language <LANG> = "english,french,german"
}

gflags::define! {
    /// Search for patterns from the given file, with one pattern per line.
    -f, --file: &Path
}

gflags::define! {
    --color <WHEN>: Color = Color::Auto
}

gflags::define! {
    -h, --help = false
}

#[derive(Debug)]
enum Color {
    Never,
    Always,
    Auto,
}

impl Value for Color {
    fn parse(arg: Arg) -> Result<Self> {
        match arg.get_str() {
            "never" => Ok(Color::Never),
            "always" => Ok(Color::Always),
            "auto" => Ok(Color::Auto),
            _ => Err(Error::new("invalid color")),
        }
    }
}

fn main() {
    let args = gflags::parse();

    if help.FLAG {
        print_help_and_exit();
    }

    println!("big_menu = {}", big_menu.FLAG);
    println!("language = {}", language.FLAG);
    if file.is_present() {
        println!("file = {}", file.FLAG.display());
    }
    println!("color = {:?}", color.FLAG);
    println!("args = {:?}", args);
}

fn print_help_and_exit() -> ! {
    println!(
        "{name} version {version}\n{authors}\n{about}",
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        authors = env!("CARGO_PKG_AUTHORS"),
        about = ABOUT,
    );
    gflags::print_help_and_exit(0);
}
