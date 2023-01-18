/*!
Utilities for parsing catalog data files based on the [nom](https://docs.rs/nom/latest/nom/) parser.

> NOTE: still under construction!
 */

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, one_of},
    combinator::recognize,
    error::ParseError,
    multi::{many0, many0_count, many1, separated_list0},
    sequence::{delimited, pair, terminated},
    IResult,
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

/** Parse a decimal integer.

<https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#decimal>
*/

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
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
    let (input, str_list) = separated_list0(tag(","), ws(decimal))(input)?;
    let (input, _) = ws(char(']'))(input)?;

    Ok((input, str_list))
}
