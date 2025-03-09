use nom::{combinator::map, IResult, Parser};

use crate::{
  combinator::into_failure,
  data_type::LengthPrefixedString,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.2.2.2 `StringValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct StringValueWithCode<'i>(LengthPrefixedString<'i>);

impl<'i> StringValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, _) = PrimitiveType::String.parse(input).map_err(|err| {
      into_failure(err).map(|err| error_position!(err.input, ExpectedPrimitive(PrimitiveType::String)))
    })?;

    map(LengthPrefixedString::parse, Self).parse(input)
  }

  #[inline]
  pub fn as_str(&self) -> &'i str {
    self.0.as_str()
  }
}

impl<'s> From<LengthPrefixedString<'s>> for StringValueWithCode<'s> {
  fn from(s: LengthPrefixedString<'s>) -> Self {
    Self(s)
  }
}
