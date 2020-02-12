use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    error::{context, ParseError},
    multi::{count, separated_list},
    number::complete::float,
    sequence::tuple,
    IResult,
};

use crate::helpers::{indent, integer, resource_path, string};
use crate::types::{Fleet, Planet, Tribute};
use crate::DataError;

pub fn parse_planet<'a>(input: &'a str) -> IResult<&'a str, Planet<'a>, DataError<&'a str>> {
    let (input, (_, _, name, _)) = context(
        "planet tag",
        tuple((tag("planet"), space1, string, line_ending)),
    )(input)?;

    let mut builder = crate::types::PlanetBuilder::default();
    builder.name(name);
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(
            1,
            attributes,
            separated_list(space1, string),
            input,
            builder
        );
        crate::parse_item_in_loop!(1, landscape, resource_path, input, builder);
        crate::parse_item_in_loop!(1, government, string, input, builder);
        crate::parse_items_in_loop!(1, description, string, input, builder);
        crate::parse_item_in_loop!(1, music, resource_path, input, builder);
        crate::parse_items_in_loop!(1, spaceport, string, input, builder);
        crate::parse_items_in_loop!(1, shipyard, string, input, builder);
        crate::parse_items_in_loop!(1, outfitter, string, input, builder);
        crate::parse_item_in_loop!(1, bribe, float, input, builder);
        crate::parse_item_in_loop!(1, security, float, input, builder);
        crate::parse_item_in_loop!(
            1,
            required_reputation,
            "\"required reputation\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, tribute, parse_tribute, input, builder);

        break;
    }

    builder
        .build()
        .map(|planet| (input, planet))
        .map_err(|error| {
            nom::Err::Failure(DataError::DataBuilderError {
                input,
                error,
                data_type: String::from("planet"),
            })
        })
}

fn parse_tribute<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Tribute<'a>, E> {
    let (input, (value, _)) = tuple((integer, line_ending))(input)?;

    let (input, (threshold, fleet)) = permutation((parse_threshold, parse_fleet))(input)?;

    Ok((
        input,
        Tribute {
            threshold,
            value,
            fleet,
        },
    ))
}

fn parse_fleet<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Fleet<'a>, E> {
    let (input, (_, _, _, _, kind, _, count, _)) = tuple((
        indent,
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

crate::parse_item_with_indent!(2, parse_threshold, threshold, integer, u32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_planet() {
        let data = r#"planet MyPlanet
	attributes a1 a2 a3
	landscape flyover/sea1
	description `This is a "special" planet`
	description `	It can have a complete description`
	spaceport `And also a spaceport!`
	shipyard "Some Ships"
	shipyard "Also Those Ships"
	outfitter "Basic Outifts"
	outfitter "Advanced Outfits"
	bribe 0.01
	security 0.5
	tribute 1000
		threshold 3000
		fleet "Impressive Fleet" 18
"#;

        let parsed = dbg!(parse_planet(&data));
        assert!(parsed.is_ok());
        let planet = parsed.unwrap().1;

        assert_eq!(planet.name, "MyPlanet");
        assert_eq!(planet.attributes, vec!["a1", "a2", "a3"]);
        assert_eq!(planet.landscape, Some("flyover/sea1"));
        assert_eq!(
            planet
                .description
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
                .join("\n"),
            String::from(
                r#"This is a "special" planet
	It can have a complete description"#
            )
        );
        assert_eq!(
            planet
                .spaceport
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
                .join("\n"),
            String::from("And also a spaceport!")
        );
        assert_eq!(planet.shipyard, vec!["Some Ships", "Also Those Ships"]);
        assert_eq!(planet.outfitter, vec!["Basic Outifts", "Advanced Outfits"]);
        assert_eq!(planet.bribe, Some(0.01));
        assert_eq!(planet.security, Some(0.5));
        assert_eq!(
            planet.tribute,
            Some(Tribute {
                threshold: 3000,
                value: 1000,
                fleet: Fleet {
                    kind: "Impressive Fleet",
                    count: 18,
                }
            })
        )
    }
}
