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

    fn outputs(self) -> OutputCounts {
        // turn the `HashMap` into an iterator but preserving the order of the keys as stored in `order`
        OutputCounts {
            iter: self.order.into_iter(),
            counts: self.seen,
        }
    }
}

#[derive(Debug)]
struct OutputCounts {
    iter: std::vec::IntoIter<String>,
    counts: HashMap<String, usize>,
}

fn format_key_val(key: &str, val: usize) -> String {
    format!("{val:>4} {key}")
}

impl Iterator for OutputCounts {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(key) = self.iter.next() {
            if let Some(&count) = self.counts.get(&key) {
                return Some(format_key_val(&key, count));
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
        .outputs()
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
}
