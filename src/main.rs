use prost::Message;
use prost_reflect::DescriptorPool;
use std::fs;

// Include the generated modules
pub mod proto;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.size = proto::common::Size::Large as i32;
    shirt
}

pub fn read_small_shirt(filepath: String) -> items::Shirt {
    let descriptor_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/descriptors.bin"));
    let pool = DescriptorPool::decode(descriptor_bytes.as_ref()).expect("Failed to decode descriptor pool");
    let message_descriptor: prost_reflect::MessageDescriptor = pool.get_message_by_name("snazzy.items.Shirt")
        .expect("Failed to get message descriptor");
    let tile_data_str = fs::read_to_string(filepath).expect("Failed to read textproto file.");
    let dynamic_message =
        prost_reflect::DynamicMessage::parse_text_format(message_descriptor, &tile_data_str).expect("Failed to parse text format");
    dynamic_message.transcode_to::<items::Shirt>().expect("Unable to transcode message to shirt.")
}

pub fn serialize_shirt(shirt: &items::Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &[u8]) -> Result<items::Shirt, prost::DecodeError> {
    items::Shirt::decode(buf)
}

fn main() {
    println!("Hello, world!");
    let large_shirt = create_large_shirt("red".to_string());
    println!("Created shirt: color={}, size={}", large_shirt.color, large_shirt.size);
    let small_shirt = read_small_shirt("src/items.textproto".to_string());
    println!("Created shirt: color={}, size={}", small_shirt.color, small_shirt.size);
}