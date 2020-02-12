use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    error::{context, ParseError},
    multi::count,
    number::complete::float,
    sequence::tuple,
    IResult,
};

use crate::helpers::{date, indent, integer, string};
use crate::types::{Account, Date, Mortgage, Start};

pub fn parse_start<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Start<'a>, E> {
    let (input, _) = tuple((tag("start"), line_ending))(input)?;
    let (input, (system, planet, date, set, account)) = permutation((
        parse_system,
        parse_planet,
        parse_date,
        parse_set,
        parse_account,
    ))(input)?;

    Ok((
        input,
        Start {
            date,
            system,
            planet,
            account,
            set,
        },
    ))
}

fn parse_account<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Account, E> {
    let (input, _) = tuple((indent, tag("account"), line_ending))(input)?;
    let (input, (credits, score, mortgage)) =
        permutation((parse_credits, parse_score, parse_mortgage))(input)?;

    Ok((
        input,
        Account {
            credits,
            score,
            mortgage,
        },
    ))
}

fn parse_mortgage<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Mortgage, E> {
    let (input, _) = tuple((
        indent,
        indent,
        tag("mortgage"),
        space1,
        tag("Mortgage"),
        line_ending,
    ))(input)?;

    let (input, (principal, interest, term)) =
        permutation((parse_principal, parse_interest, parse_term))(input)?;

    Ok((
        input,
        Mortgage {
            principal,
            interest,
            term,
        },
    ))
}

crate::parse_item_with_indent!(1, parse_system, system, string, &'a str);
crate::parse_item_with_indent!(1, parse_set, set, string, &'a str);
crate::parse_item_with_indent!(1, parse_planet, planet, string, &'a str);
crate::parse_item_with_indent!(1, parse_date, date, date, Date);

crate::parse_item_with_indent!(2, parse_credits, credits, integer, u64);
crate::parse_item_with_indent!(2, parse_score, score, integer, u32);

crate::parse_item_with_indent!(3, parse_principal, principal, integer, u64);
crate::parse_item_with_indent!(3, parse_interest, interest, float, f32);
crate::parse_item_with_indent!(3, parse_term, term, integer, u16);

#[cfg(test)]
mod test {
    use super::*;

    use nom::error::VerboseError;

    #[test]
    fn can_parse_start() {
        let data = r#"start
	system "my system"
	planet "this planet"
    date 01 07 2020
	set "my license"
	account
		credits 5000
		score 100
		mortgage Mortgage
			principal 33333
			interest 0.005
            term 365
"#;

        let parsed = dbg!(parse_start::<VerboseError<&str>>(&data));
        assert!(parsed.is_ok());
        let start = parsed.unwrap().1;
        assert_eq!(start.system, String::from("my system"));
        assert_eq!(start.planet, String::from("this planet"));
        assert_eq!(start.set, String::from("my license"));
        assert_eq!(
            start.date,
            Date {
                day: 1,
                month: 7,
                year: 2020
            }
        );
        assert_eq!(
            start.account,
            Account {
                credits: 5000,
                score: 100,
                mortgage: Mortgage {
                    principal: 33333,
                    interest: 0.005,
                    term: 365
                }
            }
        );
    }
}
