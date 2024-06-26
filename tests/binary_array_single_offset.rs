use nrbf::{RemotingMessage, Value};

#[test]
fn binary_array_single_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      3,
      0x01, 0x00, 0x00, 0x00,
      0x0A, 0x00, 0x00, 0x00,
      0xD0, 0x07, 0x00, 0x00,
      0,
      8,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    11
  ];

  let output = RemotingMessage::Value(Value::Array(vec![
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
    Value::Int32(0),
  ]));

  assert_eq!(RemotingMessage::parse(&input), Ok(output));
}
