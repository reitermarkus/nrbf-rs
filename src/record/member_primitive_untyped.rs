use nom::{
  IResult, Parser,
  combinator::{fail, map},
};

use crate::{
  Value,
  combinator::into_failure,
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int8, Int16, Int32, Int64, Single, TimeSpan, UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
  error::Error,
  value,
};

/// 2.5.2 `MemberPrimitiveUnTyped`
#[derive(Debug, Clone, PartialEq)]
pub enum MemberPrimitiveUnTyped {
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
}

impl MemberPrimitiveUnTyped {
  pub fn parse(input: &[u8], primitive_type: PrimitiveType) -> IResult<&[u8], Self, Error<'_>> {
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
      PrimitiveType::Null => fail().parse(input).map_err(into_failure),
      PrimitiveType::String => fail().parse(input).map_err(into_failure),
    }
  }

  #[inline]
  pub(crate) fn into_value(self) -> Value<'static> {
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
    }
  }
}
