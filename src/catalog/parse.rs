/*!
Utilities for parsing catalog data files based on the [nom](https://docs.rs/nom/latest/nom/) parser.

> NOTE: still under construction!
 */

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, alphanumeric1, char, digit1, i32, multispace0},
    combinator::{map_parser, recognize},
    error::ParseError,
    multi::{many0_count, separated_list0},
    number::complete::double,
    sequence::tuple,
    sequence::{delimited, pair},
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

/** Parse a Rust-style identifier.

<https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#rust-style-identifiers>
*/
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/// Parse a list of positive integers of the form `[0, 1, 2]`.
pub fn int_list(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = ws(char('['))(input)?;
    let (input, str_list) = separated_list0(tag(","), ws(digit1))(input)?;
    let (input, _) = ws(char(']'))(input)?;

    Ok((input, str_list))
}

/// Parsed [sexagesimal](https://en.wikipedia.org/wiki/Sexagesimal) number.
#[derive(Debug)]
#[allow(dead_code)] // FIXME
pub struct ParsedSexagesimal {
    major: i32,
    minor: i32,
    second: f64,
}

// TODO: make a higher order function that can take a delimiter

/// Inspired by <https://stackoverflow.com/a/41362344>
macro_rules! sexagesimal_delim {
    ($(#[$attr:meta])* => $name:ident, $delim:literal) => {
        $(#[$attr])*
        pub fn $name(input: &str) -> IResult<&str, ParsedSexagesimal> {
            let (input, (major, _, minor, _, second)) =
                tuple((i32, char($delim), i32, char($delim), double))(input)?;

            Ok((
                input,
                ParsedSexagesimal {
                    major,
                    minor,
                    second,
                },
            ))
        }
    }
}

sexagesimal_delim! {
    /// Parse a sexagesimal number of the form `-12_34_56.7890`.
    => sexagesimal_underscore, '_'
}

sexagesimal_delim! {
    /// Parse a sexagesimal number of the form `-12 34 56.7890`.
    => sexagesimal_space, ' '
}

macro_rules! sexagesimal_compact {
    ($(#[$attr:meta])* => $name:ident, $t1:literal, $t2: literal, $t3: literal) => {
        $(#[$attr])*
        pub fn $name(input: &str) -> IResult<&str, ParsedSexagesimal> {
            let (input, major) = map_parser(take($t1), i32)(input)?;
            let (input, minor) = map_parser(take($t2), i32)(input)?;
            let (input, second) = map_parser(take($t3), double)(input)?;

            Ok((
                input,
                ParsedSexagesimal {
                    major,
                    minor,
                    second,
                },
            ))
        }
    }
}

sexagesimal_compact! {
    /// Parse a degree of the form `-DDMMSS`.
    => degree_compact, 3usize, 2usize, 2usize
}

sexagesimal_compact! {
    /// Parse an hour of the form `HHMMSS.S`.
    => hour_compact, 2usize, 2usize, 4usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sex() {
        sexagesimal_underscore("12_34_56.7890").unwrap();
        sexagesimal_underscore("-12_34_56.7890").unwrap();
        sexagesimal_space("12 34 56.7890").unwrap();
        sexagesimal_space("-12 34 56.7890").unwrap();
        degree_compact("+444022").unwrap();
        degree_compact("-444022").unwrap();
        hour_compact("000509.9").unwrap();
    }
}
