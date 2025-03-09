use nom::{IResult, Parser, combinator::map, number::complete::le_i64};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{Error, error_position},
};

/// 2.1.1 `INT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int64(pub i64);

impl Int64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_i64, Self).parse(input).map_err(|err| {
      into_failure(err)
        .map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Int64)))
    })
  }
}

impl_primitive!(Int64, i64, visit_i64, deserialize_i64);
