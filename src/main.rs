#[derive(Debug, PartialEq)]
struct IntMessage {
    a: u64 // tag number = 1
}

impl IntMessage {
    fn from_bytes(slice_bytes: &[u8]) -> IntMessage {
        let mut res = IntMessage{a: 0};
        let key = unpack_key(slice_bytes);
        let next_pos = next_varint_pos(slice_bytes, 0);
        if key.wire_type == WireType::Varint {
            if key.tag == 1 {
                res.a = varint_decode(&slice_bytes[(next_pos as usize)..]);
            }
        }
        res
    }
}

#[derive(Debug, PartialEq)]
struct Key {
    wire_type: WireType,
    tag: u64,
}

fn next_varint_pos(slice: &[u8], start: u64) -> u64 {
    let mut scanned = start;
    for byte in slice {
        scanned +=1;
        if byte & 0b10000000 == 0 {
            break;
        }
    }
    scanned
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

fn unpack_key(varint_key: &[u8]) -> Key {
    let decoded_key = varint_decode(varint_key);
    let wire_type = match (decoded_key as u8) & 0b00000111 {
        0 => WireType::Varint,
        _ => panic!(),
    };
    Key{wire_type: wire_type, tag: decoded_key >> 3}
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
    // let intout: u64 = varint_decode(&[0b10101100, 0b00000010]);
    // println!("{:?}", intout);
    // let varint = varint_encode(300);
    // println!("{:?}", IntMessage::from_bytes(&*varint));
    let mut slice = Vec::new();
    slice.extend_from_slice(&*varint_encode(1 << 3 | 0));
    slice.extend_from_slice(&*varint_encode(2));
    println!("{:?}", slice);
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
    assert_eq!(*varint_encode(42), [0b00101010]);
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
    assert_eq!(unpack_key(&*varint_encode(1 << 3 | 0)), Key{wire_type: WireType::Varint, tag: 1});
}

#[test]
fn can_decode_intmessage() {
    let mut message_bytes = Vec::new();
    message_bytes.extend_from_slice(&*varint_encode(1 << 3 | 0));
    message_bytes.extend_from_slice(&*varint_encode(42));
    assert_eq!(IntMessage::from_bytes(&*message_bytes.into_boxed_slice()), IntMessage{a: 42});
}