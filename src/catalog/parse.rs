/*!
Utilities for parsing catalog data files based on the [nom](https://docs.rs/nom/latest/nom/) parser.

> NOTE: still under construction!
 */

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, i32},
    number::complete::double,
    combinator::recognize,
    error::ParseError,
    multi::{many0_count, separated_list0},
    sequence::{delimited, pair},
    IResult,
    sequence::tuple,
};

/**
A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
trailing whitespace, returning the output of `inner`.

<https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser>
*/
pub fn ws<'a, F, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/** Parse a Rust-style identifier.

<https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#rust-style-identifiers>
*/
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/// Parse a list of positive integers of the form `[0, 1, 2]`
pub fn int_list(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = ws(char('['))(input)?;
    let (input, str_list) = separated_list0(tag(","), ws(digit1))(input)?;
    let (input, _) = ws(char(']'))(input)?;

    Ok((input, str_list))
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedSexagesimal {
    major: i32,
    minor: i32,
    second: f64
}

pub fn sexagesimal(input: &str) -> IResult<&str, ParsedSexagesimal> {
    let (input, (major, _, minor, _, second)) = tuple((ws(i32), char('_'), i32, char('_'), double))(input)?;

    Ok((input, ParsedSexagesimal{major, minor, second}))
}

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn test_sex() {
    sexagesimal(" 43_56_19.123456").unwrap();
    sexagesimal("-43_56_19.123456").unwrap();
}

}