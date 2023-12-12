use byteorder::ReadBytesExt;
use byteorder::{LittleEndian, WriteBytesExt};
use std::collections::HashMap;
use std::io::{Read, Write};

use crate::core_app::traits::Compressor;

pub struct LzwCompressor {
    dictionary: HashMap<Vec<u8>, u128>,
}

impl Compressor for LzwCompressor {
    fn new() -> Self {
        let mut dictionary = HashMap::new();

        for i in 0..=255 {
            dictionary.insert(vec![i as u8], i);
        }

        LzwCompressor { dictionary }
    }

    fn compress<R: Read, W: Write>(&mut self, input: R, output: W) -> std::io::Result<()> {
        let mut input = input;
        let mut output = output;
        self.reset_dictionary();
        let mut current_code = 256u128;
        let mut current_sequence = Vec::new();
        while let Ok(byte) = input.read_u8() {
            current_sequence.push(byte);

            if !self.dictionary.contains_key(&current_sequence) {
                match output.write_u128::<LittleEndian>(
                    *self
                        .dictionary
                        .get(&current_sequence[..current_sequence.len() - 1])
                        .unwrap(),
                ) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                self.dictionary.insert(current_sequence, current_code);
                current_code += 1;

                current_sequence = vec![byte];
            }
        }

        output.write_u128::<LittleEndian>(*self.dictionary.get(&current_sequence).unwrap_or(&0))?;
        Ok(())
    }

    fn decompress<R: Read, W: Write>(&mut self, input: R, output: W) -> std::io::Result<()> {
        let mut input = input;
        let mut output = output;
        self.reset_dictionary();
        let mut inverse_dict: HashMap<u128, Vec<u8>> = self
            .dictionary
            .iter()
            .map(|(key, value)| (*value, key.clone()))
            .collect();

        let mut current_code = 256u128;
        let mut previous_code = None;
        let mut previous_sequence = Vec::new();

        while let Ok(code) = input.read_u128::<LittleEndian>() {
            if code == 0 && previous_sequence.is_empty() {
                continue;
            }
            let sequence = match inverse_dict.get(&code) {
                Some(s) => s.clone(),
                None if code == current_code => {
                    let mut s = previous_sequence.clone();
                    s.push(previous_sequence[0]);
                    s
                }
                None => {
                    let mut s = previous_sequence.clone();
                    s.push(previous_sequence[0]);
                    inverse_dict.insert(current_code, s.clone());
                    s
                }
            };

            for byte in &sequence {
                output.write_u8(*byte)?;
            }

            if let Some(previous_code) = previous_code {
                let mut new_sequence = inverse_dict.get(&previous_code).unwrap().clone();
                new_sequence.push(sequence[0]);
                inverse_dict.insert(current_code, new_sequence.clone());
                current_code += 1;
            }

            previous_code = Some(code);
            previous_sequence = sequence;
        }

        Ok(())
    }
}

impl LzwCompressor {
    fn reset_dictionary(&mut self) {
        self.dictionary.clear();
        for i in 0..=255 {
            self.dictionary.insert(vec![i as u8], i);
        }
    }
}
