use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Default)]
struct LineCounts {
    /// What entries have been seen and how many times
    seen: HashMap<String, usize>,
    /// ordering of what was seen
    order: Vec<String>,
}

impl LineCounts {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, key: &str) {
        if let Some(val) = self.seen.get_mut(key) {
            *val += 1;
        } else {
            self.seen.insert(key.to_string(), 1);
            self.order.push(key.to_string());
        }
    }

    fn into_counted_lines(self) -> CountedLines {
        // turn the `HashMap` into an iterator but preserving the order of the keys as stored in `order`
        CountedLines {
            iter: self.order.into_iter(),
            counts: self.seen,
        }
    }

    fn into_unique_lines(self) -> UniqueLines {
        UniqueLines {
            iter: self.order.into_iter(),
            counts: self.seen,
        }
    }
}

pub trait UniqOutput {
    fn format_key_val(key: &str, val: Option<&usize>) -> Option<String>;
}

#[derive(Debug)]
struct CountedLines {
    iter: std::vec::IntoIter<String>,
    counts: HashMap<String, usize>,
}

impl UniqOutput for CountedLines {
    fn format_key_val(key: &str, val: Option<&usize>) -> Option<String> {
        if let Some(&count) = val {
            Some(format!("{count:>4} {key}"))
        } else {
            None
        }
    }
}

impl Iterator for CountedLines {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(key) = self.iter.next() {
            Self::format_key_val(&key, self.counts.get(&key))
        } else {
            None
        }
    }
}

// TODO: this can be written better with traits. The same underlying structure is the same
#[derive(Debug)]
struct UniqueLines {
    iter: std::vec::IntoIter<String>,
    counts: HashMap<String, usize>,
}

impl UniqOutput for UniqueLines {
    fn format_key_val(key: &str, val: Option<&usize>) -> Option<String> {
        if let Some(&count) = val {
            if count == 1 {
                return Some(key.to_string());
            }
        }
        None
    }
}

impl Iterator for UniqueLines {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        for key in self.iter.by_ref() {
            if let Some(x) = Self::format_key_val(&key, self.counts.get(&key)) {
                return Some(x);
            }
        }
        None
    }
}

pub fn line_counts<R>(reader: BufReader<R>) -> impl Iterator<Item = String>
where
    R: Read,
{
    reader
        .lines()
        .map(|l| l.unwrap())
        .fold(LineCounts::new(), |mut acc, line| {
            acc.add(&line);
            acc
        })
        .into_counted_lines()
}

pub fn unique_lines<R>(reader: BufReader<R>) -> impl Iterator<Item = String>
where
    R: Read,
{
    reader
        .lines()
        .map(|l| l.unwrap())
        .fold(LineCounts::new(), |mut acc, line| {
            acc.add(&line);
            acc
        })
        .into_unique_lines()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_count_lines_simple() {
        let reader = BufReader::new(Cursor::new("hello\nhello\nhi".to_string()));
        let outputs: Vec<String> = line_counts(reader).collect();
        assert_eq!(outputs, vec!["   2 hello", "   1 hi"]);
    }
    #[test]
    fn test_unique_lines() {
        let reader = BufReader::new(Cursor::new("hello\nhello\nhi".to_string()));
        let outputs: Vec<String> = unique_lines(reader).collect();
        assert_eq!(outputs, vec!["hi"]);
    }
}
