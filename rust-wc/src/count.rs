use core::str;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::ops;

fn open_file(file_name: &str) -> Result<File> {
    let file = File::open(file_name)?;
    Ok(file)
}

fn get_word_count_in_buffer(buffer: &str, in_word: bool) -> (usize, bool) {
    let boundary_characters = [b'\n', b' ', b'\r', b'\t'];
    let mut word_count = 0;
    let mut on_word = in_word;
    for c in buffer.as_bytes() {
        if boundary_characters.contains(c) {
            on_word = false;
        } else if !on_word {
            on_word = true;
            word_count += 1;
        }
    }
    (word_count, on_word)
}

fn get_character_count(buffer: &str) -> usize {
    buffer.char_indices().count()
}

fn use_reader<R>(reader: &mut BufReader<R>, result: &mut ResultOutput)
where
    R: Sized + std::io::Read,
{
    let mut in_word = false;
    let mut buffer = String::new();
    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(x) => {
                result.line_count += 1;
                result.character_count += get_character_count(&buffer);
                let (word_count, now_in_word) = get_word_count_in_buffer(&buffer, in_word);
                result.word_count += word_count;
                result.byte_count += x;
                in_word = now_in_word;
                buffer.clear();
            }
            Err(e) => panic!("{e}"),
        }
    }
}
pub fn get_counts(file_name: String) -> Result<ResultOutput> {
    if file_name == "-" {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut result = ResultOutput::new(String::new());
        use_reader(&mut reader, &mut result);
        Ok(result)
    } else {
        let file = open_file(&file_name)?;
        let mut result = ResultOutput::new(file_name);
        let mut reader = BufReader::new(file);
        use_reader(&mut reader, &mut result);
        Ok(result)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResultOutput {
    pub name: String,
    pub line_count: usize,
    pub word_count: usize,
    pub byte_count: usize,
    pub character_count: usize,
}

impl ResultOutput {
    pub fn new(file_name: String) -> Self {
        Self {
            name: file_name,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ResultOutput {
    //      12      36     237 Cargo.toml
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:>8}{:>8}{:>8}{:>8} {file_name}",
            self.line_count,
            self.word_count,
            self.byte_count,
            self.character_count,
            file_name = self.name
        )
    }
}

impl ops::AddAssign<Self> for ResultOutput {
    fn add_assign(&mut self, rhs: Self) {
        self.byte_count += rhs.byte_count;
        self.line_count += rhs.line_count;
        self.word_count += rhs.word_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multibyte_count() {
        let sample = "™";
        let buffer = sample.as_bytes();
        assert_eq!(sample.len(), 3);
        assert_eq!(buffer.len(), 3);
        assert_eq!(get_character_count(sample), 1);
    }
}
