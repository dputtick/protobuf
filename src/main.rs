fn varint_decode(bytes: &[u8]) -> u64 {
    // bytes = 1010 1100 0000 0010 -> 300
    let bytes_iter = bytes.into_iter();
    let retval: u64 = bytes_iter.take_while(|&byte| byte & 0b10000000 == 0b10000000)
        .map(|byte| byte & 0b01111111)
        .fold(0, |acc, byte| acc + byte);
    retval
}

fn main() {
    let varint: u64 = varint_decode(&[0b10101100, 0b00000010]);
    println!("{:?}", varint);
}

#[test]
fn can_decode_varint() {
    assert_eq!(varint_decode(&[0b10101100, 0b00000010]), 300);
}


// read most significant bit
// clear most significant bit
// read it as a number
// iterate through bytes, removing first digit until we hit a 0
// reverse order
// concatenate
