use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::{opt, peek},
    error::context,
    multi::{count, many0, many1},
    number::complete::float,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::helpers::{indent, integer, resource_path, string};
use crate::types::{Ship, ShipAttributes, ShipWeapon, Sprite};
use crate::DataError;

pub fn parse_ship<'a>(input: &'a str) -> IResult<&'a str, Ship<'a>, DataError<&'a str>> {
    let (input, (_, _, name, subclass, _)) = context(
        "ship tag",
        tuple((
            tag("ship"),
            space1,
            string,
            opt(preceded(space1, string)),
            line_ending,
        )),
    )(input)?;

    let mut builder = crate::types::ShipBuilder::default();
    builder.name(name);
    builder.subclass(subclass);
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(1, plural, string, input, builder);
        crate::parse_item_in_loop!(1, sprite, parse_sprite, input, builder);
        crate::parse_item_in_loop!(1, thumbnail, resource_path, input, builder);
        crate::parse_item_in_loop!(1, attributes, parse_ship_attributes, input, builder);
        crate::parse_item_in_loop!(1, outfits, parse_outfits, input, builder);
        crate::parse_items_in_loop!(
            1,
            engine,
            |input| tuple((
                separated_pair(float, space1, float),
                opt(preceded(space1, float))
            ))(input)
            .map(|(r, (p, o))| (r, (p.0, p.1, o))),
            input,
            builder
        );
        crate::parse_items_in_loop!(
            1,
            gun,
            |input| tuple((
                separated_pair(float, space1, float),
                opt(preceded(space1, string))
            ))(input)
            .map(|(remaining, ((v0, v1), l))| (remaining, (v0, v1, l))),
            input,
            builder
        );
        crate::parse_items_in_loop!(
            1,
            turret,
            |input| tuple((
                separated_pair(float, space1, float),
                opt(preceded(space1, string))
            ))(input)
            .map(|(remaining, ((v0, v1), l))| (remaining, (v0, v1, l))),
            input,
            builder
        );
        crate::parse_items_in_loop!(
            1,
            fighter,
            |input| tuple((
                separated_pair(float, space1, float),
                opt(preceded(space1, string))
            ))(input)
            .map(|(remaining, ((v0, v1), l))| (remaining, (v0, v1, l))),
            input,
            builder
        );
        crate::parse_items_in_loop!(
            1,
            drone,
            |input| tuple((
                separated_pair(float, space1, float),
                opt(preceded(space1, string))
            ))(input)
            .map(|(remaining, ((v0, v1), l))| (remaining, (v0, v1, l))),
            input,
            builder
        );
        crate::parse_items_in_loop!(
            1,
            leak,
            |input| separated_pair(string, space1, separated_pair(integer, space1, integer))(input)
                .map(|(remaining, (l, (v0, v1)))| (remaining, (l, v0, v1))),
            input,
            builder
        );
        crate::parse_items_in_loop!(
            1,
            explode,
            separated_pair(string, space1, integer),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            final_explode,
            "\"final explode\"",
            string,
            input,
            builder
        );
        crate::parse_items_in_loop!(1, description, string, input, builder);

        break;
    }

    builder.build().map(|ship| (input, ship)).map_err(|error| {
        nom::Err::Failure(DataError::DataBuilderError {
            input,
            error,
            data_type: String::from("ship"),
        })
    })
}

pub fn parse_sprite<'a>(input: &'a str) -> IResult<&'a str, Sprite<'a>, DataError<&'a str>> {
    let (input, name) = context("sprite", resource_path)(input)?;

    let peeked: IResult<_, _, (&str, nom::error::ErrorKind)> = peek(preceded(
        tuple((line_ending, count(indent, 2))),
        tag("\"frame time\""),
    ))(input);

    if peeked.is_ok() {
        let (input, frame_time) = preceded(
            tuple((line_ending, count(indent, 2), tag("\"frame time\""), space1)),
            integer,
        )(input)?;
        let (input, delay) = preceded(
            tuple((line_ending, count(indent, 2), tag("\"delay\""), space1)),
            integer,
        )(input)?;
        let (input, random_start_frame) = opt(preceded(
            tuple((line_ending, count(indent, 2))),
            tag("\"random start frame\""),
        ))(input)?;

        let (input, _) = line_ending(input)?;

        Ok((
            input,
            Sprite::Sprite {
                name,
                frame_time,
                delay,
                random_start_frame: random_start_frame.is_some(),
            },
        ))
    } else {
        Ok((input, Sprite::Simple(name)))
    }
}

pub fn parse_ship_attributes<'a>(
    input: &'a str,
) -> IResult<&'a str, ShipAttributes<'a>, DataError<&'a str>> {
    let (input, _) = context("ship attributes", line_ending)(input)?;

    let mut builder = crate::types::ShipAttributesBuilder::default();
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(
            2,
            licenses,
            preceded(
                line_ending,
                many1(preceded(count(indent, 3), terminated(string, line_ending)))
            ),
            input,
            builder
        );
        crate::parse_item_in_loop!(2, category, string, input, builder);
        crate::parse_item_in_loop!(2, cost, "\"cost\"", integer, input, builder);
        crate::parse_item_in_loop!(2, cost, integer, input, builder);
        crate::parse_item_in_loop!(2, shields, "\"shields\"", integer, input, builder);
        crate::parse_item_in_loop!(2, hull, "\"hull\"", integer, input, builder);
        crate::parse_item_in_loop!(
            2,
            automaton,
            "\"automaton\"",
            |input| integer(input).map(|(r, v): (_, u32)| (r, v != 0)),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            required_crew,
            "\"required crew\"",
            integer,
            input,
            builder
        );
        crate::parse_item_in_loop!(2, bunks, "\"bunks\"", integer, input, builder);
        crate::parse_item_in_loop!(2, mass, "\"mass\"", integer, input, builder);
        crate::parse_item_in_loop!(2, drag, "\"drag\"", float, input, builder);
        crate::parse_item_in_loop!(
            2,
            heat_dissipation,
            "\"heat dissipation\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            fuel_capacity,
            "\"fuel capacity\"",
            integer,
            input,
            builder
        );
        crate::parse_item_in_loop!(2, cargo_space, "\"cargo space\"", integer, input, builder);
        crate::parse_item_in_loop!(2, outfit_space, "\"outfit space\"", integer, input, builder);
        crate::parse_item_in_loop!(
            2,
            weapon_capacity,
            "\"weapon capacity\"",
            integer,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            engine_capacity,
            "\"engine capacity\"",
            integer,
            input,
            builder
        );
        crate::parse_item_in_loop!(2, weapon, parse_ship_weapon, input, builder);

        break;
    }

    builder
        .build()
        .map(|ship_attributes| (input, ship_attributes))
        .map_err(|error| {
            nom::Err::Failure(DataError::DataBuilderError {
                input,
                error,
                data_type: String::from("ship attributes"),
            })
        })
}

pub fn parse_ship_weapon<'a>(input: &'a str) -> IResult<&'a str, ShipWeapon, DataError<&'a str>> {
    let (input, _) = context("ship attributes - weapon", line_ending)(input)?;

    let mut builder = crate::types::ShipWeaponBuilder::default();
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(3, blast_radius, "\"blast radius\"", integer, input, builder);
        crate::parse_item_in_loop!(
            3,
            shield_damage,
            "\"shield damage\"",
            integer,
            input,
            builder
        );
        crate::parse_item_in_loop!(3, hull_damage, "\"hull damage\"", integer, input, builder);
        crate::parse_item_in_loop!(3, hit_force, "\"hit force\"", integer, input, builder);

        break;
    }

    builder
        .build()
        .map(|ship_weapon| (input, ship_weapon))
        .map_err(|error| {
            nom::Err::Failure(DataError::DataBuilderError {
                input,
                error,
                data_type: String::from("ship attributes - weapon"),
            })
        })
}

pub fn parse_outfits<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<(&'a str, u32)>, DataError<&'a str>> {
    preceded(
        line_ending,
        many1(terminated(
            preceded(
                count(indent, 2),
                tuple((string, opt(preceded(space1, integer)))),
            ),
            many1(tuple((many0(indent), line_ending))),
        )),
    )(input)
    .map(|(remaining, outfits)| {
        (
            remaining,
            outfits
                .into_iter()
                .map(|(o, c)| (o, c.unwrap_or(1)))
                .collect::<Vec<(&'a str, u32)>>(),
        )
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn can_parse_ship() {
        let data = r#"ship "Shuttle"
    sprite "ship/shuttle"
    thumbnail "thumbnail/shuttle"
    attributes
        category "Transport"
        "cost" 100000
        "shields" 1000
        "hull" 100
        "required crew" 1
        "bunks" 2
        "mass" 50
        "drag" 1
        "heat dissipation" 1
        "fuel capacity" 500
        "cargo space" 20
        "outfit space" 100
        "weapon capacity" 0
        "engine capacity" 60
        weapon
            "blast radius" 10
            "shield damage" 100
            "hull damage" 50
            "hit force" 200
    outfits
        "Fuel Cell"
        "Battery Pack"
        "Shield Generator"

        "Fuel Thruster"
        "Fuel Steering"
        "Hyperdrive"

    engine -5 50
    engine 5 50
    gun 0 -30
    leak "leak" 50 50
    explode "explosion" 10
    description "My Shuttle."
    description `   It doesn't do much.`
"#;

        let parsed = dbg!(super::parse_ship(data));
        assert!(parsed.is_ok());
        let ship = parsed.unwrap().1;

        assert_eq!(ship.name, "Shuttle");
        assert_eq!(ship.subclass, None);
        assert_eq!(ship.plural, None);
        assert_eq!(ship.sprite, crate::Sprite::Simple("ship/shuttle"));
        assert_eq!(ship.thumbnail, "thumbnail/shuttle");

        assert_eq!(
            ship.attributes,
            crate::ShipAttributes {
                licenses: vec![],
                category: "Transport",
                cost: 100000,
                shields: 1000,
                hull: 100,
                automaton: false,
                required_crew: 1,
                bunks: 2,
                mass: 50,
                drag: 1.0,
                heat_dissipation: 1.0,
                fuel_capacity: 500,
                cargo_space: 20,
                outfit_space: 100,
                weapon_capacity: 0,
                engine_capacity: 60,
                weapon: crate::ShipWeapon {
                    blast_radius: 10,
                    shield_damage: 100,
                    hull_damage: 50,
                    hit_force: 200,
                },
            }
        );
        assert_eq!(
            ship.outfits,
            vec![
                ("Fuel Cell", 1),
                ("Battery Pack", 1),
                ("Shield Generator", 1),
                ("Fuel Thruster", 1),
                ("Fuel Steering", 1),
                ("Hyperdrive", 1)
            ]
        );
        assert_eq!(ship.engine, vec![(-5.0, 50.0, None), (5.0, 50.0, None)]);
        assert_eq!(ship.gun, vec![(0.0, -30.0, None)]);
        assert_eq!(ship.turret, vec![]);
        assert_eq!(ship.fighter, vec![]);
        assert_eq!(ship.drone, vec![]);
        assert_eq!(ship.leak, vec![("leak", 50, 50)]);
        assert_eq!(ship.explode, vec![("explosion", 10)]);
        assert_eq!(ship.final_explode, None);
        assert_eq!(
            ship.description,
            vec!["My Shuttle.", "   It doesn\'t do much."]
        );
    }
}
