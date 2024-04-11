use serde_json::{Map, Value as JsonValue};
use nom::{
  branch::alt,
  bytes::complete::{escaped, is_not, take_while},
  character::complete::{char, multispace1, one_of},
  combinator::{cut, map, opt, recognize},
  error::{context, ContextError, ParseError},
  multi::{many0, separated_list0},
  sequence::{delimited, preceded, separated_pair, terminated}, IResult, Parser,
};

pub fn parse_script(script: &str) -> Map<String, JsonValue> {
  preceded(bom, map(keys_and_values, to_hashmap))(script).unwrap().1
}

fn bom(input: &str) -> IResult<&str, Option<char>> {
  opt(char('\u{feff}'))(input)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E> {
  separated_pair(delimited(sp, parse_str, sp), delimited(sp, char('='), sp), delimited(sp, json_value, sp)).parse(i)
}

fn keys_and_values<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<(&'a str, JsonValue)>, E> {
  many0(delimited(sp, key_value, sp))(i)
}

fn to_hashmap(tuple_vec: Vec<(&str, JsonValue)>) -> Map<std::string::String, JsonValue> {
  let mut map = Map::new();
  for (k, v) in tuple_vec {
      map.insert(k.to_string(), v);
  }
  map
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Map<std::string::String, JsonValue>, E> {
  context(
    "map",
    preceded(
      preceded(sp, char('{')),
      cut(terminated(
        map(
          keys_and_values,
          to_hashmap,
        ),
        preceded(sp, char('}')),
      )),
    ),
  )(i)
}

fn sp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
  recognize(many0(alt((
      multispace1,
      comment_line,
  ))))(input)
}

fn comment_line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
  recognize(preceded(
      char('#'),
      take_while(|c| c != '\n')
  ))(input)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  escaped(one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789:_"), '\\', one_of("\"n\\"))(i)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> {
  context(
    "string",
    cut(preceded(sp, cut(terminated(parse_str, sp)))),
  )(i)
}

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<JsonValue>, E> {
  context(
    "array",
    delimited(
      char('{'),
      preceded(
          sp,
          terminated(
            map(
              separated_list0(multispace1, is_not(" =}")),
              |vec_str: Vec<&str>| {
                vec_str.into_iter().map(|s| JsonValue::String(s.to_string())).collect()
              },
            ),
              sp,
          ),
      ),
      char('}'),
  ),
  )(i)
}

fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, JsonValue, E> {
  preceded(
    sp,
    alt((
      map(array, JsonValue::Array),
      map(hash, JsonValue::Object),
      map(string, |s| JsonValue::String(String::from(s)))
    )),
  )
  .parse(i)
}
