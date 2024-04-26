use serde_json::Value as JsonValue;
use nom::{
  branch::alt,
  bytes::complete::{escaped, is_not, take_while},
  character::complete::{char, multispace1, one_of, space0},
  combinator::{cut, map, opt, recognize},
  error::{context, ContextError, ParseError},
  multi::{many0, separated_list0},
  sequence::{delimited, preceded, terminated}, IResult, Parser,
};

pub fn parse_script(script: &str) -> JsonValue {
  convert_vector_of_tuples_to_vector_of_vectors(preceded(bom, keys_and_values)(script).unwrap().1)
}

fn bom(input: &str) -> IResult<&str, Option<char>> {
  opt(char('\u{feff}'))(input)
}

fn key_value<'a, E>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E>
where
  E: ParseError<&'a str> + ContextError<&'a str>,
{
  let comparison_op = one_of("=><");
  let (i, key) = delimited(sp, parse_str, sp)(i)?;
  let (i, _) = delimited(sp, space0, comparison_op)(i)?;
  let (i, _) = delimited(sp, space0, opt(char('=')))(i)?;
  let (i, value) = delimited(sp, json_value, sp)(i)?;
  Ok((i, (key, value)))
}

fn keys_and_values<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<(&'a str, JsonValue)>, E> {
  many0(delimited(sp, key_value, sp))(i)
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<(&str, JsonValue)>, E> {
  context(
    "map",
    preceded(
      preceded(sp, char('{')),
      cut(terminated(
          keys_and_values,
        preceded(sp, char('}')),
      )),
    ),
  )(i)
}

fn sp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
  recognize(many0(alt((
      multispace1,
      comment_line,
      recognize(char('"'))
  ))))(input)
}

fn comment_line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
  recognize(preceded(
      char('#'),
      take_while(|c| c != '\n')
  ))(input)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  escaped(one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789:_/.-"), '\\', one_of("\"n\\"))(i)
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
              separated_list0(multispace1, is_not(" =}\n\t")),
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
      map(hash, |vec| convert_vector_of_tuples_to_vector_of_vectors(vec)),
      map(string, |s| JsonValue::String(String::from(s)))
    )),
  )
  .parse(i)
}

fn convert_tuple_to_array(tuple: (&str, JsonValue)) -> JsonValue {
  JsonValue::Array(vec![JsonValue::String(tuple.0.to_string()), tuple.1])
}

fn convert_vector_of_tuples_to_vector_of_vectors(array: Vec<(&str, JsonValue)>) -> JsonValue {
  JsonValue::Array(array.into_iter().map(convert_tuple_to_array).collect())
}
