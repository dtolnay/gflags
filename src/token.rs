use std::env::{self, ArgsOs};
use std::ffi::OsString;

pub struct Tokenizer {
    args: ArgsOs,
    pending: String,
    pending_index: usize,
    rest_are_args: bool,
}

pub enum Token {
    Short(char),
    Long(String),
    Arg(OsString),
}

impl Tokenizer {
    pub(crate) fn new() -> Self {
        let mut args = env::args_os();

        // Skip the executable.
        let _ = args.next();

        Tokenizer {
            args,
            pending: String::new(),
            pending_index: 0,
            rest_are_args: false,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.pending_index < self.pending.len() {
            let ch = self.pending[self.pending_index..].chars().next().unwrap();
            self.pending_index += ch.len_utf8();
            return Some(Token::Short(ch));
        }

        let arg = self.args.next()?;
        if self.rest_are_args {
            return Some(Token::Arg(arg));
        }

        let mut string = match arg.into_string() {
            Ok(string) => string,
            Err(non_string) => return Some(Token::Arg(non_string)),
        };

        if string == "--" {
            self.rest_are_args = true;
            return self.args.next().map(Token::Arg);
        }

        if string.starts_with("--") {
            string.replace_range(..2, "");
            return Some(Token::Long(string));
        }

        if string.starts_with("-") && string != "-" {
            let ch = string[1..].chars().next().unwrap();
            self.pending = string;
            self.pending_index = 1 + ch.len_utf8();
            return Some(Token::Short(ch));
        }

        return Some(Token::Arg(OsString::from(string)));
    }

    pub fn next_arg(&mut self) -> Option<OsString> {
        if self.pending_index < self.pending.len() {
            Some(OsString::from(self.pending.split_off(self.pending_index)))
        } else {
            self.args.next()
        }
    }
}
