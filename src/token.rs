use std::ffi::OsStr;

pub struct Tokenizer {
    args: argv::Iter,
    pending: &'static str,
    pending_index: usize,
    rest_are_args: bool,
}

pub enum Token {
    Short(char),
    Long(&'static str),
    Arg(&'static OsStr),
}

impl Tokenizer {
    pub(crate) fn new() -> Self {
        let mut args = argv::iter();

        // Skip the executable.
        let _ = args.next();

        Tokenizer {
            args,
            pending: "",
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

        let string = match arg.to_str() {
            Some(string) => string,
            None => return Some(Token::Arg(arg)),
        };

        if string == "--" {
            self.rest_are_args = true;
            return self.args.next().map(Token::Arg);
        }

        if string.starts_with("--") {
            return Some(Token::Long(&string[2..]));
        }

        if string.starts_with("-") && string != "-" {
            let ch = string[1..].chars().next().unwrap();
            self.pending = string;
            self.pending_index = 1 + ch.len_utf8();
            return Some(Token::Short(ch));
        }

        return Some(Token::Arg(OsStr::new(string)));
    }

    pub fn next_arg(&mut self) -> Option<&'static OsStr> {
        if self.pending_index < self.pending.len() {
            let rest = &self.pending[self.pending_index..];
            self.pending_index = self.pending.len();
            Some(OsStr::new(rest))
        } else {
            self.args.next()
        }
    }
}
