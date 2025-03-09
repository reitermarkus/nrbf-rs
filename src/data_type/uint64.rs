use nom::{IResult, Parser, combinator::map, number::complete::le_u64};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{Error, error_position},
};

/// 2.1.1 `UINT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt64(pub u64);

impl UInt64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_u64, Self).parse(input).map_err(|err| {
      into_failure(err)
        .map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::UInt64)))
    })
  }
}

impl_primitive!(UInt64, u64, visit_u64, deserialize_u64);
