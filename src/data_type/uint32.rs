use nom::{IResult, Parser, combinator::map, number::complete::le_u32};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{Error, error_position},
};

/// 2.1.1 `UINT32`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt32(pub u32);

impl UInt32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_u32, Self).parse(input).map_err(|err| {
      into_failure(err)
        .map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::UInt32)))
    })
  }
}

impl_primitive!(UInt32, u32, visit_u32, deserialize_u32);
