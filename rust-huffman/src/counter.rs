use std::{collections::HashMap, io::BufRead};

#[derive(Debug)]
pub struct Characters<R: BufRead> {
    /// Source of our characters
    reader: R,
    /// counts
    pub counted: HashMap<char, usize>,
}

impl<R: BufRead> Characters<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            counted: HashMap::new(),
        }
    }
    fn count_line(&mut self, line: &str) {
        line.chars().for_each(|c| {
            self.counted
                .insert(c, *self.counted.get(&c).unwrap_or(&0_usize) + 1);
        });
    }
    pub fn read_all(&mut self) -> std::io::Result<HashMap<char, usize>> {
        let mut buffer = String::new();
        loop {
            match self.reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    self.count_line(&buffer);
                    buffer.clear();
                }
                Err(e) => return Err(e),
            }
        }
        Ok(self.counted.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs::File, io::BufReader, path::PathBuf};
    fn get_local_path() -> PathBuf {
        let mut cwd = match env::current_dir() {
            Ok(x) => x,
            Err(e) => panic!("{e}"),
        };
        if !cwd.ends_with("rust-huffman") {
            cwd.push("rust-huffman");
        }
        cwd
    }
    #[test]
    fn test_count_simple() {
        let source = "aaabbc";
        let mut chars = Characters::new(source.as_bytes());

        let result = chars.read_all();
        assert!(result.is_ok());
        assert_eq!(
            chars.counted,
            HashMap::from_iter([('a', 3), ('b', 2), ('c', 1)])
        );
    }
    #[test]
    fn test_count_les_mis() {
        let mut package_path = get_local_path();
        package_path.push("static/les-mis.txt");
        let mut chars = Characters::new(BufReader::new(File::open(package_path).unwrap()));

        let result = chars.read_all();
        assert!(result.is_ok());
        assert_eq!(chars.counted.get(&'X').unwrap(), &333);
        assert_eq!(chars.counted.get(&'t').unwrap(), &223000);
    }
}
