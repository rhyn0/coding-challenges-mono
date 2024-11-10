use std::{
    collections::HashMap,
    io::{self, BufRead},
};
use thiserror::Error;

#[derive(Default)]
pub struct EncodingResult {
    pub data: Vec<u8>,
    pub padding: Option<u8>,
}

impl EncodingResult {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Unknown character for encoding '{0:?}'")]
    UnknownCharacter(char),
    #[error("Error reading from input file")]
    FailedRead(#[from] io::Error),
}

pub fn encode<R>(
    mut reader: R,
    mapping: &HashMap<char, String>,
) -> Result<EncodingResult, EncodingError>
where
    R: BufRead,
{
    let mut buffer = String::new();
    let _ = reader.read_to_string(&mut buffer)?;
    let mut output = EncodingResult::new();
    let mut current_byte = 0u8;
    let mut remaining_bits = 8u8;
    for c in buffer.chars() {
        if let Some(code) = mapping.get(&c) {
            for bit in code.chars() {
                let bit_value = u8::from(bit == '1');
                current_byte = (current_byte << 1) | bit_value;

                remaining_bits -= 1;

                // If current_byte is full, push it to encoded data and reinitialize our monitors
                if remaining_bits == 0 {
                    output.data.push(current_byte);
                    current_byte = 0;
                    remaining_bits = 8;
                }
            }
        } else {
            return Err(EncodingError::UnknownCharacter(c));
        }
    }
    if remaining_bits > 0 {
        output.data.push(current_byte);
    }
    output.padding = Some(remaining_bits);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use io::Write;

    use super::*;
    use crate::huffman::HuffmanTree;
    use std::fs::{File, OpenOptions};

    #[test]
    fn test_encodings() {
        let counts = HashMap::from_iter([('a', 3), ('b', 2), ('c', 1)]);
        let tree = HuffmanTree::new(counts);
        let encodings = tree.get_huffman_codes();
        assert_eq!(
            encodings,
            HashMap::from_iter([
                ('a', "0".to_string()),
                ('b', "11".to_string()),
                ('c', "10".to_string()),
            ])
        );
    }
    #[test]
    fn test_encode_bit_string() {
        let expected = "aaabbc";
        let counts = HashMap::from_iter([('a', 3), ('b', 2), ('c', 1)]);
        let huffman_codes = HuffmanTree::new(counts.clone()).get_huffman_codes();
        let encoded = encode(std::io::Cursor::new(expected), &huffman_codes).unwrap();
        assert_eq!(encoded.data, vec![0b00011111, 0b00000000,]);
    }
    #[test]
    fn test_consistent_leaf_codes() {
        // make sure that the codes generated are repeatable and have one solution.
        // this specifically makes sure that leafs ordering is consistent
        // input has 2 leafs of the same frequency which need to be consistent
        let counts = HashMap::from_iter([('a', 5), ('b', 5), ('c', 10), ('d', 20)]);
        let huffman_codes = HuffmanTree::new(counts.clone()).get_huffman_codes();
        assert_eq!(
            huffman_codes,
            HashMap::from_iter([
                ('a', "110".to_string()),
                ('b', "111".to_string()),
                ('c', "10".to_string()),
                ('d', "0".to_string()),
            ])
        );
    }
    #[test]
    fn test_consistent_internal_codes() {
        // make sure that the codes generated are repeatable and have one solution.
        // this specifically makes sure that internal node ordering is consistent
        // input will create two internal nodes of same weight that must be equal
        let counts = HashMap::from_iter([('a', 5), ('b', 5), ('c', 5), ('d', 5)]);
        let huffman_codes = HuffmanTree::new(counts.clone()).get_huffman_codes();
        assert_eq!(
            huffman_codes,
            HashMap::from_iter([
                ('a', "00".to_string()),
                ('b', "01".to_string()),
                ('c', "10".to_string()),
                ('d', "11".to_string()),
            ])
        );
    }
}
