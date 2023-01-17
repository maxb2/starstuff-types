use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of, multispace0},
    combinator::recognize,
    multi::{many0, many0_count, many1, separated_list0},
    sequence::{ pair, terminated, delimited},
    IResult,
    error::ParseError,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

#[derive(Debug)]
pub struct Constellation {
    name: String,
    lines: Vec<Vec<u32>>
}

pub fn int_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = ws(char('['))(input)?;
    let (input, str_list) = separated_list0(tag(","), ws(decimal))(input)?;
    let (input, _) = ws(char(']'))(input)?;

    let mut _int_list: Vec<u32> = vec![];

    for s in str_list {
        _int_list.push(s.parse::<u32>().unwrap())
    }

    Ok((input, _int_list))
}

pub fn record(input: &str) -> IResult<&str, Constellation> {
    let (input, name) = ws(identifier)(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, lines) = separated_list0(ws(char(';')), int_list)(input)?;

    match input {
        "" => Ok((input, Constellation {name: String::from(name), lines})),
        _ => panic!("found extra input while parsing: \"{}\"", input)
        // TODO: get this error working
        // _ => Err(nom::Err::Error(("123def", nom::error::ErrorKind::Alpha))) // NOTE: not working
    }
    
}

fn main() {
    let rec = "Psc = [123, 32131, 321321]; [31312, 54654645]  dfgsdgf   ";
    println!("{:#?}", record(&rec));
}
