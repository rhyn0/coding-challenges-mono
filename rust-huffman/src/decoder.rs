use std::{collections::HashMap, io::BufRead, num};

use crate::huffman;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Huffman header is not in expected format.")]
    InvalidHeader,
    #[error("Huffman header does not have a valid integer value - {0}")]
    InvalidHeaderValue(#[from] num::ParseIntError),
    #[error("Invalid padding value - {0:?}")]
    InvalidPaddingString(String),
    #[error("Unable to read from file")]
    FailedToRead,
}

type HeaderReturn = (HashMap<char, usize>, u8);

fn parse_table(content: &str) -> Result<HashMap<char, usize>, DecodeError> {
    if let Some(no_prefix) = content.strip_prefix('{') {
        if let Some(inner) = no_prefix.strip_suffix('}') {
            let mut hash_value = HashMap::new();
            for pair in inner.split(", ") {
                let (key, value) = pair.split_once(": ").unwrap();
                let character = match key.trim_matches('\'') {
                    r"\r" => '\r',
                    r"\t" => '\t',
                    r"\n" => '\n',
                    "\\" => '\\',
                    r"\'" => '\'',
                    "\\\"" => '\"',
                    r"\0" => '\0',
                    x => x.chars().next().unwrap(),
                };
                let code = match value.trim_matches('"').parse::<usize>() {
                    Ok(x) => x,
                    Err(e) => return Err(DecodeError::InvalidHeaderValue(e)),
                };
                hash_value.insert(character, code);
            }

            Ok(hash_value)
        } else {
            // not looking like debug format of HashMap
            Err(DecodeError::InvalidHeader)
        }
    } else {
        // not looking like debug format of HashMap
        Err(DecodeError::InvalidHeader)
    }
}

fn get_table_padding(content: &str) -> Result<HeaderReturn, DecodeError> {
    let (table, padding) = match content.split_once("::") {
        Some((first, second)) => {
            let trim_second = second.trim_end();
            if trim_second.len() != 1 {
                return Err(DecodeError::InvalidPaddingString(second.to_string()));
            }
            (first, trim_second)
        }
        // invalid header content
        None => return Err(DecodeError::InvalidHeader),
    };
    let padding_value = match padding.parse::<u8>() {
        Ok(x) => x,
        Err(e) => return Err(DecodeError::InvalidPaddingString(e.to_string())),
    };
    let hash_value = match parse_table(table) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok((hash_value, padding_value))
}
pub fn get_huffman_table(header_line: &str) -> Result<HeaderReturn, DecodeError> {
    let header_chars = header_line.chars();
    let header_len_string = header_chars.clone().take(32).collect::<String>();
    let header_length = match header_len_string.parse::<usize>() {
        Ok(x) => x,
        Err(e) => return Err(DecodeError::InvalidHeaderValue(e)),
    };
    if header_line.len() > 32 + header_length {
        // more characters in header than expected
        return Err(DecodeError::InvalidHeader);
    }
    let header_content = header_chars
        .skip(32)
        .take(header_length)
        .collect::<String>();
    get_table_padding(&header_content)
}

pub fn decode<R>(
    mut reader: R,
    mapping: HashMap<char, usize>,
    padding: u8,
) -> Result<String, DecodeError>
where
    R: BufRead,
{
    let tree = huffman::HuffmanTree::new(mapping);
    let root = tree.build_tree();
    let mut current_node = &root;
    let mut buffer = Vec::new();
    if reader.read_to_end(&mut buffer).is_err() {
        return Err(DecodeError::FailedToRead);
    }

    let final_idx = buffer.len() - 1;
    let mut output = String::new();
    for (idx, byte) in buffer.iter().enumerate() {
        let bit_string = if idx == final_idx {
            format!("{byte:0width$b}", width = 8 - (padding as usize))
        } else {
            format!("{byte:08b}")
        };
        for bit in bit_string.chars() {
            current_node = match current_node {
                huffman::HuffmanNode::Internal(_, left, right) => {
                    if bit == '0' {
                        left
                    } else {
                        right
                    }
                }
                huffman::HuffmanNode::Leaf(_, _) => unreachable!(),
            };
            if let huffman::HuffmanNode::Leaf(ch, _) = current_node {
                output.push(*ch);
                current_node = &root;
            };
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use crate::{encoder, huffman};
    #[test]
    fn test_decode() {
        let expected = "aaabbc";
        let counts: HashMap<char, usize> = HashMap::from_iter([('a', 3), ('b', 2), ('c', 1)]);
        let huffman_codes = huffman::HuffmanTree::new(counts.clone()).get_huffman_codes();
        let encoded = encoder::encode(std::io::Cursor::new(expected), &huffman_codes).unwrap();
        let encoded_string = String::from_utf8_lossy(&encoded.data);
        assert_eq!(encoded_string.bytes().collect::<Vec<_>>(), encoded.data);
        let result = decode(Cursor::new(encoded_string.to_string()), counts, 7);
        assert_eq!(result.unwrap(), expected);
    }
}
