use nom::{IResult, Parser, combinator::map, number::complete::le_f64};

use super::impl_primitive;
use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{Error, error_position},
};

/// 2.1.1.2 `Double`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Double(pub f64);

impl Double {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    map(le_f64, Self).parse(input).map_err(|err| {
      into_failure(err)
        .map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitive(PrimitiveType::Double)))
    })
  }
}

impl_primitive!(Double, f64, visit_f64, deserialize_f64);
