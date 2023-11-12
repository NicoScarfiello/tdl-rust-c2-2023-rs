use tdl_rust_c2_2023_rs::lzw::lzw;

const STRING_TO_COMPRESS: &str = "ABCAAABBBBCCCCCCC";

fn main() {
    let mut compressor = lzw::LzwCompressor::new();

    let mut compressed = Vec::new();

    compressor
        .compress(STRING_TO_COMPRESS.as_bytes(), &mut compressed)
        .unwrap();

    println!("Original: {}", STRING_TO_COMPRESS);
    println!("Compressed: {:?}", compressed);
}
