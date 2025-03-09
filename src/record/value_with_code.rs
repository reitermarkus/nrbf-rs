use nom::{IResult, Parser, combinator::map};

use crate::{
  Value,
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int8, Int16, Int32, Int64, LengthPrefixedString, Single, TimeSpan,
    UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
  error::Error,
  value,
};

/// 2.2.2.1 `ValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub enum ValueWithCode<'i> {
  Boolean(Boolean),
  Byte(Byte),
  Char(Char),
  Decimal(Decimal),
  Double(Double),
  Int16(Int16),
  Int32(Int32),
  Int64(Int64),
  SByte(Int8),
  Single(Single),
  TimeSpan(TimeSpan),
  DateTime(DateTime),
  UInt16(UInt16),
  UInt32(UInt32),
  UInt64(UInt64),
  Null,
  String(LengthPrefixedString<'i>),
}

impl<'i> ValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, primitive_type) = PrimitiveType::parse(input)?;

    match primitive_type {
      PrimitiveType::Boolean => map(|input| Boolean::parse(input), Self::Boolean).parse(input),
      PrimitiveType::Byte => map(|input| Byte::parse(input), Self::Byte).parse(input),
      PrimitiveType::Char => map(|input| Char::parse(input), Self::Char).parse(input),
      PrimitiveType::Decimal => map(|input| Decimal::parse(input), Self::Decimal).parse(input),
      PrimitiveType::Double => map(|input| Double::parse(input), Self::Double).parse(input),
      PrimitiveType::Int16 => map(|input| Int16::parse(input), Self::Int16).parse(input),
      PrimitiveType::Int32 => map(|input| Int32::parse(input), Self::Int32).parse(input),
      PrimitiveType::Int64 => map(|input| Int64::parse(input), Self::Int64).parse(input),
      PrimitiveType::SByte => map(|input| Int8::parse(input), Self::SByte).parse(input),
      PrimitiveType::Single => map(|input| Single::parse(input), Self::Single).parse(input),
      PrimitiveType::TimeSpan => map(|input| TimeSpan::parse(input), Self::TimeSpan).parse(input),
      PrimitiveType::DateTime => map(|input| DateTime::parse(input), Self::DateTime).parse(input),
      PrimitiveType::UInt16 => map(|input| UInt16::parse(input), Self::UInt16).parse(input),
      PrimitiveType::UInt32 => map(|input| UInt32::parse(input), Self::UInt32).parse(input),
      PrimitiveType::UInt64 => map(|input| UInt64::parse(input), Self::UInt64).parse(input),
      PrimitiveType::Null => Ok((input, Self::Null)),
      PrimitiveType::String => map(LengthPrefixedString::parse, Self::String).parse(input),
    }
  }

  #[inline]
  pub(crate) fn into_value(self) -> Value<'i> {
    match self {
      Self::Boolean(v) => Value::Boolean(v.into()),
      Self::Byte(v) => Value::Byte(v.into()),
      Self::Char(v) => Value::Char(v.into()),
      Self::Decimal(v) => Value::Decimal(value::Decimal(v)),
      Self::Double(v) => Value::Double(v.into()),
      Self::Int16(v) => Value::Int16(v.into()),
      Self::Int32(v) => Value::Int32(v.into()),
      Self::Int64(v) => Value::Int64(v.into()),
      Self::SByte(v) => Value::SByte(v.into()),
      Self::Single(v) => Value::Single(v.into()),
      Self::TimeSpan(v) => Value::TimeSpan(value::TimeSpan(v)),
      Self::DateTime(v) => Value::DateTime(value::DateTime(v)),
      Self::UInt16(v) => Value::UInt16(v.into()),
      Self::UInt32(v) => Value::UInt32(v.into()),
      Self::UInt64(v) => Value::UInt64(v.into()),
      Self::Null => Value::Null,
      Self::String(s) => Value::String(s.as_str()),
    }
  }
}
