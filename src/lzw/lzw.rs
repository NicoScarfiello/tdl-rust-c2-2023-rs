use byteorder::ReadBytesExt;
use byteorder::{LittleEndian, WriteBytesExt};
use std::collections::HashMap;
use std::io::{Read, Write};

pub struct LzwCompressor {
    dictionary: HashMap<Vec<u8>, u16>,
}

impl LzwCompressor {
    pub fn new() -> Self {
        let mut dictionary = HashMap::new();
        for i in 0..=255 {
            dictionary.insert(vec![i as u8], i);
        }

        LzwCompressor { dictionary }
    }

    pub fn compress<R: Read, W: Write>(&mut self, input: R, output: W) -> std::io::Result<()> {
        let mut input = input;
        let mut output = output;

        let mut current_code = 256u16;
        let mut current_sequence = Vec::new();

        while let Ok(byte) = input.read_u8() {
            current_sequence.push(byte);

            if !self.dictionary.contains_key(&current_sequence) {
                match output.write_u16::<LittleEndian>(
                    *self
                        .dictionary
                        .get(&current_sequence[..current_sequence.len() - 1])
                        .unwrap(),
                ) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                self.dictionary
                    .insert(current_sequence.clone(), current_code);
                current_code += 1;

                current_sequence = vec![byte];
            }
        }

        output.write_u16::<LittleEndian>(*self.dictionary.get(&current_sequence).unwrap())?;

        Ok(())
    }
}
