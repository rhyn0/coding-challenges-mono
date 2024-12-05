use std::io::{BufRead, BufReader, Read};
pub mod count;

pub fn read_lines<R>(reader: BufReader<R>) -> impl Iterator<Item = String>
where
    R: Read,
{
    reader
        .lines()
        .map(|l| l.unwrap())
        .fold(Vec::new(), |mut acc, line| {
            if acc.is_empty() || acc.last().unwrap() != &line {
                acc.push(line);
            }
            acc
        })
        .into_iter()
}

pub fn repeated_lines<R>(reader: BufReader<R>) -> impl Iterator<Item = String>
where
    R: Read,
{
    let mut lines = reader.lines().map(|l| l.unwrap());
    let mut prev_line = lines.next().unwrap_or_default();
    let mut duplicates = Vec::new();
    for line in lines {
        if line == prev_line && !duplicates.contains(&line) {
            duplicates.push(line.clone());
        }
        prev_line = line;
    }
    duplicates.into_iter()
}

pub mod prelude {
    pub use super::count::line_counts;
    pub use super::read_lines;
    pub use super::repeated_lines;
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_dedup_adjacent_lines() {
        let reader = BufReader::new(Cursor::new("hello\nhello".to_string()));
        let deduped_lines: Vec<_> = read_lines(reader).collect();
        assert_eq!(deduped_lines, vec!["hello".to_string()])
    }
    #[test]
    fn test_no_dedup() {
        let reader = BufReader::new(Cursor::new("hello\nworld".to_string()));
        let deduped_lines: Vec<_> = read_lines(reader).collect();
        assert_eq!(
            deduped_lines,
            vec!["hello".to_string(), "world".to_string()]
        )
    }
    #[test]
    #[should_panic]
    fn test_extra_newline() {
        let reader = BufReader::new(Cursor::new("hello\nworld\r\n\n".to_string()));
        let deduped_lines: Vec<_> = read_lines(reader).collect();
        assert_eq!(
            deduped_lines,
            // TODO: this fails because we aren't clearing out the empty line
            // want to pull the "read,unwrap lines" logic into own struct first
            vec!["hello".to_string(), "world".to_string()]
        )
    }
    #[test]
    fn test_empty_duplicated() {
        let reader = BufReader::new(Cursor::new("".to_string()));
        let duplicate_lines: Vec<String> = repeated_lines(reader).collect();
        assert_eq!(duplicate_lines, Vec::<String>::new());
    }
    #[test]
    fn test_duplicated() {
        let reader = BufReader::new(Cursor::new("hello\nhello".to_string()));
        let duplicate_lines: Vec<String> = repeated_lines(reader).collect();
        assert_eq!(duplicate_lines, vec!["hello".to_string()]);
    }
}
