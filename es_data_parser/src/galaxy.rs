use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::opt,
    error::{context, ParseError},
    multi::count,
    number::complete::double,
    sequence::tuple,
    IResult,
};

use crate::helpers::{indent, resource_path, string};
use crate::types::{Galaxy, Position};

pub fn parse_galaxy<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Galaxy<'a>, E> {
    let (input, (_, _, name, _)) = context(
        "galaxy tag",
        tuple((tag("galaxy"), space1, string, line_ending)),
    )(input)?;
    let (input, (pos, sprite)) =
        context("galaxy fields", permutation((parse_pos, opt(parse_sprite))))(input)?;

    Ok((input, Galaxy { name, pos, sprite }))
}

fn parse_pos<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Position, E> {
    let (input, (_, _, _, x, _, y, _)) = tuple((
        indent,
        tag("pos"),
        space1,
        double,
        space1,
        double,
        line_ending,
    ))(input)?;

    Ok((input, Position { x, y }))
}

crate::parse_item_with_indent!(1, parse_sprite, sprite, resource_path, &'a str);

#[cfg(test)]
mod test {
    use super::*;

    use nom::error::VerboseError;

    #[test]
    fn can_parse_galaxy() {
        let data = r#"galaxy "Milky Way"
	pos -27 32.8
	sprite ui/galaxy

"#;

        let parsed = dbg!(parse_galaxy::<VerboseError<&str>>(&data));
        assert!(parsed.is_ok());
        let galaxy = parsed.unwrap().1;

        assert_eq!(galaxy.name, "Milky Way");
        assert_eq!(galaxy.pos, Position { x: -27.0, y: 32.8 });
        assert_eq!(galaxy.sprite, Some("ui/galaxy"));
    }
}
