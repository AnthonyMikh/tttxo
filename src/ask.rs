use std::io::{self, prelude::*};
use std::default::Default;
use std::str::FromStr;

pub(crate) fn ask_prompt(buf: &mut String, prompt: &'static str) -> io::Result<()> {
    io::stdout().write_all(prompt.as_bytes())?;
    io::stdout().flush()?;
    io::stdin().read_line(buf)?;
    Ok(())
}

pub(crate) fn ask_value<T: FromStr>(
    buf: &mut String,
    prompt: &'static str,
    error_msg: &'static str,
) -> io::Result<T> {
    loop {
        buf.clear();
        ask_prompt(buf, prompt)?;
        match buf.trim().parse() {
            Ok(value) => return Ok(value),
            Err(_) => println!("{}", error_msg)
        }
    }
}

pub(crate) fn ask_value_or_default<T: FromStr + Default>(
    buf: &mut String,
    prompt: &'static str,
    error_msg: &'static str,
) -> io::Result<T> {
    loop {
        buf.clear();
        ask_prompt(buf, prompt)?;
        let line = buf.trim();
        if line.is_empty() {
            return Ok(T::default())
        }
        match line.parse() {
            Ok(value) => return Ok(value),
            Err(_) => println!("{}", error_msg)
        }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum Answer {
    Yes,
    No,
}

impl Answer {
    #[allow(unused)]
    pub(crate) fn yes(&self) -> bool {
        *self == Answer::Yes
    }

    pub(crate) fn no(&self) -> bool {
        *self == Answer::No
    }
}

impl Default for Answer {
    fn default() -> Self {
        Answer::Yes
    }
}

impl FromStr for Answer {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, ()> {
        match line {
            "y" | "Y" => Ok(Answer::Yes),
            "n" | "N" => Ok(Answer::No),
            _ => Err(()),
        }
    }
}

impl From<Answer> for bool {
    fn from(answer: Answer) -> bool {
        match answer {
            Answer::Yes => true,
            Answer::No => false,
        }
    }
}

pub(crate) fn ask_yes_no(
    buf: &mut String,
    prompt: &'static str,
    error_msg: &'static str,
) -> io::Result<Answer> {
    ask_value_or_default(buf, prompt, error_msg)
}

pub(crate) fn ask_value_validated<T, VF>(
    buf: &mut String,
    prompt: &'static str,
    error_msg: &'static str,
    mut valid: VF,
    validation_msg: &'static str,
) -> io::Result<T>
where
    T: FromStr,
    VF: FnMut(&T) -> bool,
{
    loop {
        let input = ask_value(buf, prompt, error_msg)?;
        if valid(&input) {
            return Ok(input)
        } else {
            println!("{}", validation_msg);
        }
    }
}
