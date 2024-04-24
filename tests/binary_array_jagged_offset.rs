use nrbf::{RemotingMessage, Value};

#[test]
fn binary_array_jagged_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      4,
      0x01, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      208, 7, 0, 0,
      7,
      8,
      9,
        0x02, 0x00, 0x00, 0x00,
      9,
        0x03, 0x00, 0x00, 0x00,
      9,
        0x04, 0x00, 0x00, 0x00,
    15,
      0x02, 0x00, 0x00, 0x00,
      0x01, 0x00, 0x00, 0x00,
      8,
      0x01, 0x00, 0x00, 0x00,
    15,
      0x03, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      8,
      0x02, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
    15,
      0x04, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      8,
      0x04, 0x00, 0x00, 0x00,
      0x05, 0x00, 0x00, 0x00,
      0x06, 0x00, 0x00, 0x00,
    11,
  ];

  let output = RemotingMessage::Value(Value::Array(vec![
    Value::Array(vec![Value::Int32(1)]),
    Value::Array(vec![Value::Int32(2), Value::Int32(3)]),
    Value::Array(vec![Value::Int32(4), Value::Int32(5), Value::Int32(6)]),
  ]));

  assert_eq!(RemotingMessage::parse(&input), Ok(output));
}
