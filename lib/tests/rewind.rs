#![feature(proc_macro_hygiene)]

use pear::input::Text;
use pear::{macros::*, parsers::*};

type Result<'a, T> = pear::result::Result<T, Text<'a>>;

#[parser(rewind)]
fn ab<'a>(input: &mut Text<'a>) -> Result<'a, ()> {
    eat('a')?;
    eat('b')?;
    eof()?;
}

#[parser(rewind)]
fn abc<'a>(input: &mut Text<'a>) -> Result<'a, ()> {
    eat('a')?;
    eat('b')?;
    eat('c')?;
    eof()?;
}

#[parser(rewind)]
fn abcd<'a>(input: &mut Text<'a>) -> Result<'a, ()> {
    eat('a')?;
    eat('b')?;
    eat('c')?;
    eat('d')?;
    eof()?;
}

#[parser]
fn combo<'a>(input: &mut Text<'a>) -> Result<'a, &'a str> {
    switch! {
        ab() => "ab",
        abc() => "abc",
        abcd() => "abcd",
        _ => return parse_error!("not ab, abc, or abcd")
    }
}

#[test]
fn test_rewinding_ab() {
    let result = parse!(combo: &mut Text::from("ab")).unwrap();
    assert_eq!(result, "ab")
}

#[test]
fn test_rewinding_abc() {
    let result = parse!(combo: &mut Text::from("abc")).unwrap();
    assert_eq!(result, "abc")
}

#[test]
fn test_rewinding_abcd() {
    let result = parse!(combo: &mut Text::from("abcd")).unwrap();
    assert_eq!(result, "abcd")
}

#[test]
fn test_rewinding_fail() {
    let result = parse!(combo: &mut Text::from("a"));
    assert!(result.is_err());

    let result = parse!(combo: &mut Text::from("abcdef"));
    assert!(result.is_err());
}