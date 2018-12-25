use std::ffi::OsString;
use std::process;

use crate::name::Name;

/// Raw argument value given for a non-boolean flag.
pub struct Arg {
    name: Name,
    arg: OsString,
}

impl Arg {
    pub(crate) fn new(name: Name, arg: OsString) -> Self {
        Arg { name, arg }
    }

    /// Access the raw value given on the command line, which may not be legal
    /// UTF-8.
    pub fn get_raw(self) -> OsString {
        self.arg
    }

    /// Access the value assuming it is UTF-8. If not UTF-8, the process will
    /// abort with an error message.
    pub fn get_string(self) -> String {
        let name = self.name;
        match self.get_raw().into_string() {
            Ok(string) => string,
            Err(_) => {
                eprintln!("Non-unicode arg for {}", name);
                process::exit(1);
            }
        }
    }
}
