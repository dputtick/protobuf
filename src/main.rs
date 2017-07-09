#[derive(Debug)]
struct IntMessage {
    a: u64 // tag number = 1
}

impl IntMessage {
    fn from_bytes(slice_bytes: &[u8]) -> IntMessage {
        IntMessage{a: 1}
    }
}

fn varint_decode(bytes: &[u8]) -> u64 {
    let mut bytes_iter = bytes.iter();
    let mut retval: u64 = 0;
    let mut shift_width = 0;
    while let Some(& next) = bytes_iter.next() {
        let value = next & 0b01111111;
        retval += (value as u64) << shift_width;
        if next & 0b10000000 == 0b10000000 {
            shift_width += 7;
        } else {
            return retval
        }
    }
    panic!("{:?}", "all leading bits were 1");
}

#[derive(Debug, PartialEq)]
enum WireType {
    Varint,
}

fn get_wire_type(varint_key: &[u8]) -> WireType {
    match (varint_decode(varint_key) as u8) & 0b00000111 {
        0 => WireType::Varint,
        _ => panic!(),
    }
}

fn varint_encode(int: u64) -> Box<[u8]> {
    let mut res = Vec::new();
    let bottommask = 0b01111111 as u64;
    let topmask = !bottommask;
    let mut temp = int;
    let mut has_more_bits = true;
    while has_more_bits {
        has_more_bits = (temp & topmask) != 0;
        let lowest_byte = (temp & bottommask) as u8;
        res.push(set_msb(lowest_byte, has_more_bits));
        temp = temp >> 7;
    }
    res.into_boxed_slice()
}

fn set_msb(byte: u8, should_set: bool) -> u8 {
    byte | (should_set as u8) << 7
}

fn main() {
    let intout: u64 = varint_decode(&[0b10101100, 0b00000010]);
    println!("{:?}", intout);
    let varint = varint_encode(300);
    println!("{:?}", IntMessage::from_bytes(&*varint));
}

#[test]
fn can_decode_varint() {
    assert_eq!(varint_decode(&[0b10101100, 0b00000010]), 300);
    assert_eq!(varint_decode(&[0b00000001]), 1);
    assert_eq!(varint_decode(&[0b00000000]), 0);
}

#[test]
fn can_encode_varint() {
    assert_eq!(*varint_encode(300), [0b10101100, 0b00000010]);
    assert_eq!(*varint_encode(1), [0b00000001]);
    assert_eq!(*varint_encode(0), [0b00000000]);
}

#[test]
fn can_roundtrip() {
    let input: &[u64] = &[150, 1500000, 0, 2340239420394];
    for int in input {
        assert_eq!(varint_decode(&*varint_encode(*int as u64)), *int as u64);
    }
}

#[test]
fn can_get_wiretype() {
    assert_eq!(get_wire_type(&*varint_encode(1 << 3 | 0)), WireType::Varint);
}