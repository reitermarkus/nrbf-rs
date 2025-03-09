use nom::{IResult, Parser, combinator::map, number::complete::le_i16};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{Error, error_position},
};

/// 2.1.1 `INT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int16(pub i16);

impl Int16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_i16, Self).parse(input).map_err(|err| {
      into_failure(err)
        .map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Int16)))
    })
  }
}

impl_primitive!(Int16, i16, visit_i16, deserialize_i16);
