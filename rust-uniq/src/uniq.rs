use std::io::{BufRead, BufReader, Read};

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
    fn test_extra_newline() {
        let reader = BufReader::new(Cursor::new("hello\nworld\r\n\n".to_string()));
        let deduped_lines: Vec<_> = read_lines(reader).collect();
        assert_eq!(
            deduped_lines,
            vec!["hello".to_string(), "world".to_string()]
        )
    }
}
