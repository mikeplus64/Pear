use pear::input::{Text, Cursor, Extent};
use pear::{macros::*, parsers::*};

type Result<'a, T> = pear::input::Result<T, Text<'a>>;

#[parser]
fn take_until_str<'a>(input: &mut Text<'a>, s: &str) -> Result<'a, &'a str> {
    take_while_slice(|&slice| !slice.ends_with(s))?
}

#[parser]
fn test_until<'a>(input: &mut Text<'a>, s: &str, r: &str) -> Result<'a, &'a str> {
    (take_until_str(s)?, eat_slice(r)?).0
}

#[test]
fn test_while_slice() {
    let result = parse!(test_until("]]", "]"): &mut Text::from("[[ a ] b c ]]"));
    assert_eq!(result.unwrap(), "[[ a ] b c ]");

    let r = parse!(test_until("]]]", "] hi"): &mut Text::from("[[ a ]] b c ]]] hi"));
    assert_eq!(r.unwrap(), "[[ a ]] b c ]]");

    let r = parse!(test_until("]", "]] b c ]]]"): &mut Text::from("[[ a ]] b c ]]]"));
    assert_eq!(r.unwrap(), "[[ a ");
}

#[parser]
fn take_until_and_str<'a>(input: &mut Text<'a>, s: &str) -> Result<'a, &'a str> {
    if s.is_empty() {
        parse_error!("what would that mean?")?;
    }

    let slice = take_while_slice(|&slice| !slice.ends_with(s))?;
    if slice.ends_with(&s[..s.len() - 1]) {
        parse_try!(skip_any());
        &slice[..slice.len() - (s.len() - 1)]
    } else {
        slice
    }
}

#[parser]
fn test_until_and<'a, 'b>(input: &mut Text<'a>, s: &str, r: &str) -> Result<'a, &'a str> {
    (take_until_and_str(s)?, eat_slice(r)?).0
}

#[test]
fn test_while_slice_and() {
    let result = parse!(test_until_and("]]", ""): &mut Text::from("[[ a ] b c ]]"));
    assert_eq!(result.unwrap(), "[[ a ] b c ");

    let r = parse!(test_until_and("]]]", " hi"): &mut Text::from("[[ a ]] b c ]]] hi"));
    assert_eq!(r.unwrap(), "[[ a ]] b c ");

    let r = parse!(test_until_and("]", "] b c ]]]"): &mut Text::from("[[ a ]] b c ]]]"));
    assert_eq!(r.unwrap(), "[[ a ");

    let r = parse!(test_until_and("]", ""): &mut Text::from("hi"));
    assert_eq!(r.unwrap(), "hi");

    let r = parse!(test_until_and("]", ""): &mut Text::from("🐥hi"));
    assert_eq!(r.unwrap(), "🐥hi");

    let r = parse!(test_until_and("]", "] b c ]]]"): &mut Text::from("[[ 🐥 ]] b c ]]]"));
    assert_eq!(r.unwrap(), "[[ 🐥 ");
}

#[parser]
fn test_until_window<'a>(input: &mut Text<'a>, s: &str, r: &str) -> Result<'a, &'a str> {
    (take_until_slice(s)?, eat_slice(r)?).0
}

#[test]
fn test_while_slice_window() {
    let result = parse!(test_until_window("]]", "]]"): &mut Text::from("[[ a ] b c ]]"));
    assert_eq!(result.unwrap(), "[[ a ] b c ");

    let r = parse!(test_until_window("]]]", "]]] hi"): &mut Text::from("[[ a ]] b c ]]] hi"));
    assert_eq!(r.unwrap(), "[[ a ]] b c ");

    let r = parse!(test_until_window("]", "]] b c ]]]"): &mut Text::from("[[ a ]] b c ]]]"));
    assert_eq!(r.unwrap(), "[[ a ");

    let r = parse!(test_until_window("]", "]] b c ]]]"): &mut Text::from("[[ 🐥 ]] b c ]]]"));
    assert_eq!(r.unwrap(), "[[ 🐥 ");

    let r = parse!(test_until_window("]", ""): &mut Text::from("🐥hi"));
    assert_eq!(r.unwrap(), "🐥hi");
}

#[test]
fn test_window_termination() {
    let result = take_while_window(&mut Text::from("a"), 2, |_| false);
    assert_eq!(result.unwrap(), "a");

    let result = take_while_window(&mut Text::from("aa"), 2, |_| false);
    assert_eq!(result.unwrap(), "");

    let result = take_some_while_some_window(&mut Text::from("a"), 2, |_| false);
    assert!(result.is_err());

    let result = take_some_while_window(&mut Text::from("aa"), 2, |_| false);
    assert!(result.is_err());

    let result = take_while_window(&mut Text::from("aa"), 2, |_| true);
    assert_eq!(result.unwrap(), "a");

    let result = take_some_while_window(&mut Text::from("aa"), 2, |_| true);
    assert_eq!(result.unwrap(), "a");

    let result = take_while_window(&mut Text::from("aaab"), 2, |&s| s == "aa");
    assert_eq!(result.unwrap(), "aa");

    let result = take_some_while_window(&mut Text::from("aaab"), 2, |&s| s == "aa");
    assert_eq!(result.unwrap(), "aa");

    let result = take_while_some_window(&mut Text::from("aa"), 2, |_| false);
    assert_eq!(result.unwrap(), "");
}

type CResult<'a, T> = pear::input::Result<Extent<T>, Cursor<&'a str>>;

#[parser]
fn take_until_cursor_str<'a>(input: &mut Cursor<&'a str>, s: &str) -> CResult<'a, &'a str> {
    take_while_slice(|&slice| !slice.ends_with(s))?
}

#[test]
fn test_cursor() {
    let input = "abchello";
    let result = take_until_cursor_str(&mut Cursor::from(input), "hell");
    let extent = result.unwrap();
    assert_eq!(extent, "abchel");
    assert_eq!(extent.start, 0);
    assert_eq!(extent.end, 6);
    assert_eq!(extent, &input[extent.start..extent.end]);

    let input = "hellothisishe";
    let mut cursor = Cursor::from(input);

    peek_slice(&mut cursor, "hello").unwrap();

    let extent = eat_any(&mut cursor).unwrap();
    assert_eq!(extent, 'h');

    let extent = take_until_cursor_str(&mut cursor, "this").unwrap();
    assert_eq!(extent, "ellothi");
    assert_eq!(extent, &input[extent.start..extent.end]);

    let extent = take_until_cursor_str(&mut cursor, "is").unwrap();
    assert_eq!(extent, "si");
    assert_eq!(extent, &input[extent.start..extent.end]);

    println!("{:?}", cursor);
    let extent = take_while(&mut cursor, |_| true).unwrap();
    assert_eq!(extent, "she");
    assert_eq!(extent, &input[extent.start..extent.end]);
}
