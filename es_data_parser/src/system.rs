use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::opt,
    error::{context, ParseError},
    multi::count,
    multi::many0,
    number::complete::{double, float},
    sequence::tuple,
    IResult,
};

use crate::helpers::{indent, integer, resource_path, string};
use crate::types::{Asteroids, Fleet, Minables, Position, System, SystemObject, Trade};

pub fn parse_system<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, System<'a>, E> {
    let (input, (_, _, name, _)) = context(
        "systme tag",
        tuple((tag("system"), space1, string, line_ending)),
    )(input)?;
    let (
        input,
        (
            pos,
            government,
            habitable,
            belt,
            haze,
            links,
            asteroids,
            minables,
            trades,
            fleets,
            objects,
        ),
    ) = context(
        "system fields",
        permutation((
            parse_pos,
            parse_government,
            parse_habitable,
            opt(parse_belt),
            opt(parse_haze),
            many0(parse_link),
            many0(parse_asteroids),
            many0(parse_minables),
            many0(parse_trades),
            many0(parse_fleet),
            many0(parse_object),
        )),
    )(input)?;

    Ok((
        input,
        System {
            name,
            pos,
            government,
            habitable,
            asteroids,
            minables,
            haze,
            belt,
            fleets,
            links,
            trades,
            objects,
        },
    ))
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

fn parse_asteroids<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Asteroids, E> {
    let (input, (_, _, _, name, _, first_value, _, second_value, _)) = tuple((
        indent,
        tag("asteroids"),
        space1,
        string,
        space1,
        integer,
        space1,
        float,
        line_ending,
    ))(input)?;
    Ok((
        input,
        Asteroids {
            name,
            first_value,
            second_value,
        },
    ))
}
fn parse_minables<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Minables, E> {
    let (input, (_, _, _, name, _, first_value, _, second_value, _)) = tuple((
        indent,
        tag("minables"),
        space1,
        string,
        space1,
        integer,
        space1,
        float,
        line_ending,
    ))(input)?;
    Ok((
        input,
        Minables {
            name,
            first_value,
            second_value,
        },
    ))
}
fn parse_trades<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Trade, E> {
    let (input, (_, _, _, name, _, price, _)) = tuple((
        indent,
        tag("trade"),
        space1,
        string,
        space1,
        integer,
        line_ending,
    ))(input)?;
    Ok((input, Trade { name, price }))
}
fn parse_fleet<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Fleet, E> {
    let (input, (_, _, _, kind, _, count, _)) = tuple((
        indent,
        tag("fleet"),
        space1,
        string,
        space1,
        integer,
        line_ending,
    ))(input)?;
    Ok((input, Fleet { kind, count }))
}
fn parse_object<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, SystemObject, E> {
    parse_object_at_level(0, input)
}
fn parse_object_at_level<'a, E: ParseError<&'a str>>(
    level: usize,
    input: &'a str,
) -> IResult<&'a str, SystemObject, E> {
    // eprintln!("--> {} - {:?}", level, input);
    let (input, (_, _, name, _)) = context(
        "object tag",
        tuple((
            count(indent, level + 1),
            tag("object"),
            opt(tuple((space1, string))),
            line_ending,
        )),
    )(input)?;
    let (input, (sprite, distance, period, offset, objects)) = context(
        "object fields",
        permutation((
            |input| opt(tuple((count(indent, level + 1), parse_sprite)))(input),
            |input| opt(tuple((count(indent, level + 1), parse_distance)))(input),
            |input| tuple((count(indent, level + 1), parse_period))(input),
            |input| opt(tuple((count(indent, level + 1), parse_offset)))(input),
            |input| many0(|input| parse_object_at_level(level + 1, input))(input),
        )),
    )(input)?;

    Ok((
        input,
        SystemObject {
            name: name.map(|name| name.1),
            sprite: sprite.map(|sprite| sprite.1),
            distance: distance.map(|distance| distance.1),
            offset: offset.map(|offset| offset.1),
            period: period.1,
            objects,
        },
    ))
}

crate::parse_item_with_indent!(1, parse_government, government, string, &'a str);
crate::parse_item_with_indent!(1, parse_habitable, habitable, float, f32);
crate::parse_item_with_indent!(1, parse_belt, belt, integer, u32);
crate::parse_item_with_indent!(1, parse_link, link, string, &'a str);
crate::parse_item_with_indent!(1, parse_haze, haze, resource_path, &'a str);

crate::parse_item_with_indent!(1, parse_sprite, sprite, resource_path, &'a str);
crate::parse_item_with_indent!(1, parse_distance, distance, float, f32);
crate::parse_item_with_indent!(1, parse_offset, offset, float, f32);
crate::parse_item_with_indent!(1, parse_period, period, float, f32);

#[cfg(test)]
mod test {
    use super::*;

    use nom::error::VerboseError;

    #[test]
    fn can_parse_system() {
        let data = r#"system "My System"
    pos -192873.2 123.456
    government FirstEmpire
    habitable 823.12
    belt 1010
    link "Other System"
    asteroids "small rock" 1 2.222
    asteroids "large metal" 7 2.345
    minables lead 11 10
    trade Goods 100
    fleet "Small Vessel" 100
    object
        sprite planet/visual-planet
        distance 1811.79
        period 1129.48
        object Moon
            sprite moon/nice-moon
            distance 229
            period 12.994
    object
        sprite star/k5
        distance 49.335
        period 18.0618
        offset 180
"#;

        let parsed = dbg!(parse_system::<VerboseError<&str>>(&data));
        assert!(parsed.is_ok());
        let system = parsed.unwrap().1;

        assert_eq!(system.name, "My System");
        assert_eq!(
            system.pos,
            Position {
                x: -192873.2,
                y: 123.456
            }
        );
        assert_eq!(system.habitable, 823.12);
        assert_eq!(system.belt, Some(1010));
        assert_eq!(system.links, vec!["Other System"]);
        assert_eq!(
            system.asteroids,
            vec![
                Asteroids {
                    name: "small rock",

                    first_value: 1,
                    second_value: 2.222
                },
                Asteroids {
                    name: "large metal",
                    first_value: 7,
                    second_value: 2.345
                }
            ]
        );
        assert_eq!(
            system.minables,
            vec![Minables {
                name: "lead",
                first_value: 11,
                second_value: 10.0
            }]
        );
        assert_eq!(
            system.trades,
            vec![Trade {
                name: "Goods",
                price: 100
            }]
        );
        assert_eq!(
            system.fleets,
            vec![Fleet {
                kind: "Small Vessel",
                count: 100
            }]
        );

        assert_eq!(
            system.objects,
            vec![
                SystemObject {
                    name: None,
                    sprite: Some("planet/visual-planet"),
                    distance: Some(1811.79),
                    period: 1129.48,
                    offset: None,
                    objects: vec![SystemObject {
                        name: Some("Moon"),
                        sprite: Some("moon/nice-moon"),
                        distance: Some(229.0),
                        period: 12.994,
                        offset: None,
                        objects: vec![]
                    }]
                },
                SystemObject {
                    name: None,
                    sprite: Some("star/k5"),
                    distance: Some(49.335),
                    period: 18.0618,
                    offset: Some(180.0),
                    objects: vec![]
                }
            ]
        );
    }
}
