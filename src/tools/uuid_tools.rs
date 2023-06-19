use crypto::{md5::Md5, digest::Digest};
use uuid::Uuid;


pub fn uuid_from_name(input_name: String) -> String {
    uuid_from_bytes(input_name.as_bytes()).to_string()
}

pub fn uuid_from_bytes(input: &[u8]) -> Uuid {
    let mut md5 = Md5::new();

    md5.input(input);
    let mut hash = [0; 16];
    md5.result(&mut hash);
    let mut bytes = hash.to_vec();

    bytes[6] &= 0x0f;
    bytes[6] |= 0x30;
    bytes[8] &= 0x3f;
    bytes[8] |= 0x80;

    bytes.swap(6, 7);
    bytes.swap(4, 5);
    bytes.swap(0, 3);
    bytes.swap(1, 2);

    let uuid = Uuid::from_slice(&bytes).unwrap();
    uuid
}