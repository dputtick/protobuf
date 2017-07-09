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

fn varint_encode(int: u64) -> Box<[u8]> {
    Box::new([0b10101100, 0b00000010])
}

fn main() {
    let varint: u64 = varint_decode(&[0b10101100, 0b00000010]);
    println!("{:?}", varint);
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
}
