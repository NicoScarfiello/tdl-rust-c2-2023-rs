mod tests {
    use tdl_rust_c2_2023_rs::lzw::lzw::LzwCompressor;
    use std::io::Cursor;
    const LORE_IPSUM_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas eu nisi et quam fringilla hendrerit. Duis eget imperdiet risus. Vivamus tincidunt libero at metus ornare, sed lobortis justo vulputate. Nulla nec sollicitudin ex. Pellentesque pretium id tortor sit amet mollis. Praesent auctor ac elit vel lacinia. Cras mattis interdum auctor. Proin rutrum nunc vel rhoncus placerat. Quisque elementum, ex sit amet congue tincidunt, nibh ex pulvinar ante, sit amet convallis quam erat in odio. Suspendisse quis libero neque. Nulla facilisi. Pellentesque placerat bibendum euismod.

    Phasellus porta iaculis dolor. Donec ac feugiat sem. Aliquam cursus risus at mi rhoncus, et viverra nibh vulputate. Sed ac varius metus, ut semper elit. Nullam sed purus id orci sollicitudin accumsan. Praesent et tempor neque, in varius lorem. Suspendisse suscipit iaculis ipsum at egestas. Vivamus sed odio quam. Duis sed mi et urna vulputate tempus et lobortis ipsum. In fermentum ligula erat, a semper nisl lacinia sit amet. Nam eu tincidunt risus. Praesent fringilla erat id felis mattis lobortis. Vestibulum pellentesque tellus eleifend leo porta, eu tristique nisl sagittis. Mauris porttitor purus erat, non dapibus elit varius in. Aenean ipsum nibh, gravida non neque vel, tempus vulputate orci.";
    
    #[test]
    pub fn test_lzw_compress_decompress_lore_ipsum_are_equal() {
        let mut compressor = LzwCompressor::new();
        let original_data = LORE_IPSUM_TEXT;
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(original_data.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, original_data.as_bytes());
        assert_eq!(decompressed_data, original_data.as_bytes());
    }  


    #[test]
    pub fn test_lzw_compress_decompress_plain_string_are_equal() {
        let mut compressor = LzwCompressor::new();
        let original_data = "ABCAAABBBBCCCCCCC";
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(original_data.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, original_data.as_bytes());
        assert_eq!(decompressed_data, original_data.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_numbers_are_equal() {
        let mut compressor = LzwCompressor::new();
        let original_data = "1700497014";
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(original_data.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, original_data.as_bytes());
        assert_eq!(decompressed_data, original_data.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_empty_string_are_equal() {
        let mut compressor = LzwCompressor::new();
        let original_data = "";
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        compressor.compress(original_data.as_bytes(), &mut compressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        assert_ne!(compressed_data, original_data.as_bytes());
        assert_eq!(decompressed_data, original_data.as_bytes());
    }  

    #[test]
    pub fn test_lzw_compress_decompress_multiple_times_not_fails() {
        let mut compressor = LzwCompressor::new();
        let original_data = "ABCAAABBBBCCCCCCC";
        let original_data_2 = "CCCCCCCBBBBAAACBA";
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();
        let mut compressed_data_2 = Vec::new();
        let mut decompressed_data_2 = Vec::new();
        compressor.compress(original_data.as_bytes(), &mut compressed_data).unwrap();
        compressor.compress(original_data_2.as_bytes(), &mut compressed_data_2).unwrap();
        compressor.decompress(Cursor::new(&compressed_data), &mut decompressed_data).unwrap();
        compressor.decompress(Cursor::new(&compressed_data_2), &mut decompressed_data_2).unwrap();
        assert_ne!(compressed_data, original_data.as_bytes());
        assert_eq!(decompressed_data, original_data.as_bytes());
        assert_ne!(compressed_data_2, original_data_2.as_bytes());
        assert_eq!(decompressed_data_2, original_data_2.as_bytes());
    }  

}
