use crate::arg::Arg;
use crate::error::Error;
use crate::name::Name;
use crate::state::Flag;
use crate::token::Tokenizer;
use crate::value::Value;
use std::process;

pub trait Parser: Sync {
    fn parse(&self, name: Name, command_line: &mut Tokenizer);
    fn is_bool(&self) -> bool;
    fn unset_bool(&self);
}

impl<T: Value> Parser for Flag<T> {
    fn parse(&self, name: Name, command_line: &mut Tokenizer) {
        if T::IS_BOOL {
            T::set_bool(self, &true);
        } else {
            let arg = next_arg(name, command_line);
            match T::parse(arg) {
                Ok(value) => self.set(value),
                Err(err) => parse_failed(name, err),
            }
        }
    }

    fn is_bool(&self) -> bool {
        T::IS_BOOL
    }

    fn unset_bool(&self) {
        T::set_bool(self, &false);
    }
}

fn next_arg(name: Name, command_line: &mut Tokenizer) -> Arg {
    match command_line.next_arg() {
        Some(arg) => Arg::new(name, arg),
        None => {
            eprintln!("Missing value for `{}`", name);
            process::exit(1);
        }
    }
}

fn parse_failed(name: Name, err: Error) -> ! {
    eprintln!("Failed to parse `{}`: {}", name, err);
    process::exit(1);
}
