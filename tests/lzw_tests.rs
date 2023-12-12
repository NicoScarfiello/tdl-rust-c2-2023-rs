mod tests {
    use tdl_rust_c2_2023_rs::lzw::lzw::LzwCompressor;
    use std::io::Cursor;
    const LORE_IPSUM_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas eu nisi et quam fringilla hendrerit. Duis eget imperdiet risus. Vivamus tincidunt libero at metus ornare, sed lobortis justo vulputate. Nulla nec sollicitudin ex. Pellentesque pretium id tortor sit amet mollis. Praesent auctor ac elit vel lacinia. Cras mattis interdum auctor. Proin rutrum nunc vel rhoncus placerat. Quisque elementum, ex sit amet congue tincidunt, nibh ex pulvinar ante, sit amet convallis quam erat in odio. Suspendisse quis libero neque. Nulla facilisi. Pellentesque placerat bibendum euismod.

    Phasellus porta iaculis dolor. Donec ac feugiat sem. Aliquam cursus risus at mi rhoncus, et viverra nibh vulputate. Sed ac varius metus, ut semper elit. Nullam sed purus id orci sollicitudin accumsan. Praesent et tempor neque, in varius lorem. Suspendisse suscipit iaculis ipsum at egestas. Vivamus sed odio quam. Duis sed mi et urna vulputate tempus et lobortis ipsum. In fermentum ligula erat, a semper nisl lacinia sit amet. Nam eu tincidunt risus. Praesent fringilla erat id felis mattis lobortis. Vestibulum pellentesque tellus eleifend leo porta, eu tristique nisl sagittis. Mauris porttitor purus erat, non dapibus elit varius in. Aenean ipsum nibh, gravida non neque vel, tempus vulputate orci.";
    const PLAIN_STRING: &str = "ABCAAABBBBCCCCCCC";
    const PLAIN_STRING_2: &str = "ABCAAABBBBCCCCCCC";
    const NUMBERS: &str = "1700497014";
    const EMPTY_STRING: &str = "";

    #[test]
    pub fn test_lzw_compress_decompress_lore_ipsum_are_equal() {
        let mut compressor = LzwCompressor::new();
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(LORE_IPSUM_TEXT.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, LORE_IPSUM_TEXT.as_bytes());
        assert_eq!(decompressed_data, LORE_IPSUM_TEXT.as_bytes());
    }  


    #[test]
    pub fn test_lzw_compress_decompress_plain_string_are_equal() {
        let mut compressor = LzwCompressor::new();
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(PLAIN_STRING.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, PLAIN_STRING.as_bytes());
        assert_eq!(decompressed_data, PLAIN_STRING.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_numbers_are_equal() {
        let mut compressor = LzwCompressor::new();
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(NUMBERS.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, NUMBERS.as_bytes());
        assert_eq!(decompressed_data, NUMBERS.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_empty_string_are_equal() {
        let mut compressor = LzwCompressor::new();
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(EMPTY_STRING.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, EMPTY_STRING.as_bytes());
        assert_eq!(decompressed_data, EMPTY_STRING.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_multiple_times_not_fails() {
        let mut compressor = LzwCompressor::new();
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        let mut compressed_data_2 = Vec::new();
        let mut decompressed_data_2 = Vec::new();
        compressor.compress(PLAIN_STRING.as_bytes(), &mut compressed_data).unwrap();
        compressor.compress(PLAIN_STRING_2.as_bytes(), &mut compressed_data_2).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data_2), &mut decompressed_data_2).unwrap();
        assert_ne!(compressed_data, PLAIN_STRING.as_bytes());
        assert_eq!(decompressed_data, PLAIN_STRING.as_bytes());
        assert_ne!(compressed_data_2, PLAIN_STRING_2.as_bytes());
        assert_eq!(decompressed_data_2, PLAIN_STRING_2.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_big_file_not_fails() {
        let mut compressor = LzwCompressor::new();
        let original_data = generate_large_string();
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(original_data.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, original_data.as_bytes());
        assert_eq!(decompressed_data, original_data.as_bytes());
    }  

    fn generate_large_string() -> String {
        const REPEAT: usize = 1000000;
        let small_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a diam lectus. Sed sit amet ipsum mauris. Maecenas congue ligula ac quam viverra nec consectetur ante hendrerit. Donec et mollis dolor. Praesent et diam eget libero egestas mattis sit amet vitae augue.";
        small_string.repeat(REPEAT)
    }
}
