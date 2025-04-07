use jomini::{text::de::from_utf8_reader, JominiDeserialize};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, multispace0, space0},
    combinator::{cut, map, peek, recognize},
    error::ErrorKind,
    multi::{fold_many0, many0, many_till},
    sequence::{pair, preceded, tuple},
    IResult,
};
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::pdx_script_parser::{parse_str, sp};

#[derive(Clone, Debug, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct CountrySetup {
    pub base_tech: Option<String>,
    pub technologies_researched: Vec<String>,
}

impl PartialEq for CountrySetup {
    fn eq(&self, other: &Self) -> bool {
        if self.base_tech != other.base_tech {
            return false;
        }
        let mut self_techs = self.technologies_researched.clone();
        let mut other_techs = other.technologies_researched.clone();

        self_techs.sort_unstable();
        other_techs.sort_unstable();

        self_techs == other_techs
    }
}

fn default_false() -> bool {
    false
}
#[derive(JominiDeserialize)]
struct RawCountrySetup {
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_1_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_2_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_3_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_4_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_5_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_6_tech: bool,
    #[jomini(default = "default_false")]
    effect_starting_technology_tier_7_tech: bool,
    #[jomini(alias = "add_technology_researched", duplicated)]
    technologies_researched: Vec<String>,
}

#[derive(JominiDeserialize)]
struct CountrySetupFile {
    #[jomini(alias = "COUNTRIES")]
    countries: HashMap<String, RawCountrySetup>,
}

impl CountrySetup {
    pub fn new() -> CountrySetup {
        CountrySetup {
            base_tech: Some("tier_7".to_string()),
            technologies_researched: vec![],
        }
    }
    pub fn parse_map_from(path: PathBuf) -> HashMap<String, CountrySetup> {
        let mut country_setups: HashMap<String, CountrySetup> = HashMap::new();

        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if entry.extension().unwrap() != "txt" {
                continue;
            };

            let raw_country_setups: CountrySetupFile =
                from_utf8_reader(&*std::fs::read(entry).unwrap()).unwrap();

            raw_country_setups
                .countries
                .iter()
                .for_each(|(country, raw_country_setup)| {
                    let base_tech = if raw_country_setup.effect_starting_technology_tier_1_tech {
                        Some("tier_1".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_2_tech {
                        Some("tier_2".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_3_tech {
                        Some("tier_3".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_4_tech {
                        Some("tier_4".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_5_tech {
                        Some("tier_5".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_6_tech {
                        Some("tier_6".to_string())
                    } else if raw_country_setup.effect_starting_technology_tier_7_tech {
                        Some("tier_7".to_string())
                    } else {
                        None
                    };

                    let country_setup = CountrySetup {
                        base_tech,
                        technologies_researched: raw_country_setup.technologies_researched.clone(),
                    };

                    country_setups.insert(
                        country.trim_start_matches("c:").to_uppercase(),
                        country_setup,
                    );
                })
        }

        country_setups
    }

    pub fn parse_map_unprocessed_values(path: PathBuf) -> HashMap<String, String> {
        let mut country_setups: HashMap<String, String> = HashMap::new();

        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if entry.extension().unwrap() != "txt" {
                continue;
            };
            let file = &std::fs::read_to_string(entry).unwrap();
            let parsed_country_setup = countries_parser(file.trim_start_matches('\u{feff}'));

            country_setups.extend(parsed_country_setup)
        }

        country_setups
    }
}

fn nested_braces(input: &str) -> IResult<&str, &str> {
    let mut depth = 0;
    let mut start_index = 0;
    let mut end_index = 0;
    let chars = input.char_indices().peekable();

    for (i, c) in chars {
        if c == '{' {
            depth += 1;
            if depth == 1 {
                start_index = i + 1;
            }
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                end_index = i;
                break;
            }
        }
    }

    if depth == 0 && end_index > start_index {
        Ok((&input[end_index + 1..], &input[start_index..end_index]))
    } else {
        Err(nom::Err::Failure(nom::error::Error::new(
            input,
            ErrorKind::Many0,
        )))
    }
}

fn parse_key_value_pairs(input: &str) -> IResult<&str, HashMap<String, String>> {
    fold_many0(
        preceded(
            sp,
            pair(
                map(parse_str, |key: &str| {
                    key.trim_start_matches("c:").to_uppercase().to_string()
                }),
                preceded(
                    multispace0,
                    preceded(
                        alt((tag("?="), tag("="))),
                        preceded(multispace0, nested_braces),
                    ),
                ),
            ),
        ),
        HashMap::new,
        |mut acc: HashMap<_, _>, (key, value)| {
            acc.insert(key, value.to_string());
            acc
        },
    )(input)
}

#[allow(clippy::manual_try_fold)]
fn dynamic_match<'a, 'b>(
    inputs: &'b [String],
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, nom::error::Error<&'a str>> + 'b {
    move |i: &str| {
        inputs.iter().fold(
            Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Tag,
            ))),
            |acc, s| {
                if acc.is_ok() {
                    acc
                } else {
                    tag(s.as_str())(i)
                }
            },
        )
    }
}

fn remove_parsed_lines(input: &str) -> IResult<&str, Vec<&str>> {
    let parsed_lines = vec![
        "add_technology_researched".to_string(),
        "effect_starting_technology_tier_1_tech".to_string(),
        "effect_starting_technology_tier_2_tech".to_string(),
        "effect_starting_technology_tier_3_tech".to_string(),
        "effect_starting_technology_tier_4_tech".to_string(),
        "effect_starting_technology_tier_5_tech".to_string(),
        "effect_starting_technology_tier_6_tech".to_string(),
        "effect_starting_technology_tier_7_tech".to_string(),
    ];
    many0(map(
        tuple((
            recognize(many_till(
                anychar,
                peek(dynamic_match(&parsed_lines.clone())),
            )),
            dynamic_match(&parsed_lines.clone()),
            multispace0,
            tag("="),
            multispace0,
            parse_str,
            multispace0,
        )),
        |(before, _, _, _, _, _, _)| before,
    ))(input)
}

fn countries_parser(input: &str) -> HashMap<String, String> {
    let input = sp::<()>(input).unwrap().0;
    let input = preceded(
        tuple((tag("COUNTRIES"), space0, char('='), space0)),
        cut(nested_braces),
    )(input)
    .unwrap()
    .1;
    let mut input = parse_key_value_pairs(input).unwrap().1;
    let mut new_input = input.clone();

    for (key, value) in input.iter_mut() {
        let (remaining, parts) = remove_parsed_lines(value).unwrap();
        let new_value = parts.join("") + remaining;
        new_input.insert(key.to_string(), new_value);
    }

    new_input
}
