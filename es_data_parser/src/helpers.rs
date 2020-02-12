use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::{char, digit1, line_ending, space1, tab},
    combinator::cut,
    error::{context, ErrorKind, ParseError},
    sequence::{preceded, terminated, tuple},
    AsChar, IResult, InputTakeAtPosition,
};

use crate::types::Date;

fn tab_hole<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context("indent to ignore (tab)", tab)(input).map(|(remaining, _)| (remaining, ()))
}
fn four_space_hole<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context("indent to ignore (space)", tag("    "))(input).map(|(remaining, _)| (remaining, ()))
}

pub fn indent<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context("indent to ignore", alt((tab_hole, four_space_hole)))(input)
}

pub fn date<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Date, E> {
    let (input, (day, _, month, _, year)) =
        context("date", tuple((integer, space1, integer, space1, integer)))(input)?;
    Ok((input, Date { day, month, year }))
}

pub fn string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        alt((
            preceded(char('"'), cut(terminated(take_until("\""), char('"')))),
            alphanumeric_or_other_ok1,
            preceded(char('`'), cut(terminated(take_until("`"), char('`')))),
        )),
    )(input)
}

pub fn alphanumeric_or_other_ok1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '\'' || ch == '-')
        },
        ErrorKind::AlphaNumeric,
    )
}

pub fn resource_path<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "resource path",
        alt((
            preceded(char('"'), cut(terminated(take_until("\""), char('"')))),
            take_until("\n"),
        )),
    )(input)
}

pub fn integer<'a, T: std::str::FromStr, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, T, E>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    context("integer", digit1)(input).map(|(input, value)| (input, value.parse::<T>().unwrap()))
}

pub fn comment_hole<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context(
        "comment to ignore",
        alt((
            tuple((tag("//"), terminated(take_until("\n"), line_ending))),
            tuple((tag("#"), terminated(take_until("\n"), line_ending))),
        )),
    )(input)
    .map(|(remaining, _)| (remaining, ()))
}

/// helper to build function that will parse a field with an indententation
#[macro_export]
macro_rules! parse_item_with_indent {
    ($nb_ident:expr, $fn_name:ident, $tag:ident, $subparser:ident, $result:ty) => {
        fn $fn_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, $result, E> {
            let (input, (_indent, _tag, _ws, extracted, _newline)) = context(
                stringify!($tag),
                tuple((
                    count(indent, $nb_ident),
                    tag(stringify!($tag)),
                    space1,
                    $subparser,
                    line_ending,
                )),
            )(input)?;

            Ok((input, extracted))
        }
    };
}

/// will parse an item present once, peeking first if the tag is present, and continuing the loop when found
#[macro_export]
macro_rules! parse_item_in_loop {
    ($nb_ident:expr, $field:ident, $subparser:expr, $input:ident, $builder:ident) => {
        crate::parse_item_in_loop!(
            $nb_ident,
            $field,
            stringify!($field),
            $subparser,
            $input,
            $builder
        )
    };
    ($nb_indent:expr, $field:ident, $tag:expr, $subparser:expr, $input:ident, $builder:ident) => {
        let peeked: IResult<_, _, (&str, nom::error::ErrorKind)> =
            nom::combinator::peek(nom::sequence::tuple((
                nom::multi::count(indent, $nb_indent),
                nom::bytes::complete::tag($tag),
            )))($input);
        if peeked.is_ok() {
            let (remaining, extracted) = nom::error::context(
                $tag,
                nom::sequence::terminated(
                    nom::sequence::preceded(
                        nom::sequence::tuple((
                            nom::multi::count(indent, $nb_indent),
                            nom::bytes::complete::tag($tag),
                            nom::combinator::opt(space1),
                        )),
                        $subparser,
                    ),
                    nom::multi::many0(line_ending),
                ),
            )($input)?;
            $input = remaining;
            $builder.$field(extracted);
            continue;
        }
    };
}

/// will parse an item present at least once, peeking first if the tag is present, and continuing the loop when found
#[macro_export]
macro_rules! parse_items_in_loop {
    ($nb_indent:expr, $field:ident, $subparser:expr, $input:ident, $builder:ident) => {
        let peeked: IResult<_, _, (&str, nom::error::ErrorKind)> =
            nom::combinator::peek(nom::sequence::tuple((
                nom::multi::count(indent, $nb_indent),
                nom::bytes::complete::tag(stringify!($field)),
            )))($input);
        if peeked.is_ok() {
            let (remaining, extracted) = nom::error::context(
                stringify!($field),
                nom::multi::many1(nom::sequence::terminated(
                    nom::sequence::preceded(
                        nom::sequence::tuple((
                            nom::multi::count(indent, $nb_indent),
                            nom::bytes::complete::tag(stringify!($field)),
                            nom::combinator::opt(space1),
                        )),
                        $subparser,
                    ),
                    nom::sequence::tuple((
                        nom::multi::many0(indent),
                        nom::multi::many0(line_ending),
                    )),
                )),
            )($input)?;
            $input = remaining;
            $builder.$field(extracted);
            continue;
        }
    };
}
