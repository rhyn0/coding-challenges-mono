use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub struct UniqueReader<R>
where
    R: Read + 'static,
{
    /// source of data
    reader: BufReader<R>,
    /// only keep repeated group lines
    keep_repeated: bool,
    /// only keep unique group lines
    only_unique: bool,
    // ignore case of following lines
    ignore_case: bool,
}

impl<R> UniqueReader<R>
where
    R: Read + 'static,
{
    pub const fn new(reader: BufReader<R>) -> Self {
        Self {
            reader,
            keep_repeated: false,
            only_unique: false,
            ignore_case: false,
        }
    }

    /// only keep repeated lines
    /// Overrides `unique`
    pub const fn repeated(mut self) -> Self {
        self.keep_repeated = true;
        self
    }

    /// only keep unique lines
    /// Incompatible with `repeated`
    pub const fn unique(mut self) -> Self {
        self.only_unique = true;
        self
    }

    /// ignore case of following lines
    pub const fn case_insensitive(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    fn read_lines(self) -> Vec<ElementWithCount> {
        // Store the configuration values we need before moving self.reader
        let keep_repeated = self.keep_repeated;
        let only_unique = self.only_unique;
        let ignore_case = self.ignore_case;

        let mut elements: Vec<ElementWithCount> = self
            .reader
            .lines()
            .map(|l| l.expect("Read a line") + "\n")
            .filter(|l| !l.trim_end().is_empty())
            .fold(Vec::new(), |mut acc, line| {
                if acc.is_empty() {
                    acc.push((line, 1));
                    return acc;
                }

                let last = acc.last().unwrap();
                let equal = if ignore_case {
                    last.0.to_lowercase() == line.to_lowercase()
                } else {
                    last.0 == line
                };

                if equal {
                    let count = last.1 + 1;
                    acc.last_mut().unwrap().1 = count;
                } else {
                    // if the last element is not the same as the current line
                    // then add it to the list as its a new group.
                    // EXCEPT for when self.keep_repeated is true, which means we only want to keep groups that have been repeated
                    // EXCEPT for when self.only_unique is true, which means we only want to keep groups that are not repeated
                    if (keep_repeated && acc.last().unwrap().1 == 1)
                        || (only_unique && acc.last().unwrap().1 > 1)
                    {
                        acc.pop();
                    }
                    acc.push((line, 1));
                }
                acc
            });
        // handle final group with no repeats
        if self.keep_repeated && !elements.is_empty() && elements.last().unwrap().1 == 1 {
            elements.pop();
        }

        elements
    }

    pub fn into_line_counts(self) -> LineCounts {
        self.read_lines().into()
    }
}

type ElementWithCount = (String, usize);

#[derive(Debug, Default)]
pub struct LineCounts {
    /// What entries have been seen and how many times
    elements_counts: Vec<ElementWithCount>,
    /// output the counts with their corresponding elements
    with_counts: bool,
}

impl From<Vec<ElementWithCount>> for LineCounts {
    fn from(elements_counts: Vec<ElementWithCount>) -> Self {
        Self {
            elements_counts,
            with_counts: false,
        }
    }
}
impl LineCounts {
    #[allow(dead_code)]
    pub fn new(elements_counts: Vec<ElementWithCount>, with_counts: bool) -> Self {
        Self {
            elements_counts,
            with_counts,
        }
    }
    pub const fn include_counts(mut self) -> Self {
        self.with_counts = true;
        self
    }
    #[inline]
    fn format_key_val(key: &str, val: usize, with_counts: bool) -> String {
        if with_counts {
            format!("{val:>4} {key}")
        } else {
            key.to_string()
        }
    }
    pub fn into_lines(self) -> impl Iterator<Item = String> {
        self.elements_counts
            .into_iter()
            .map(move |(key, val)| Self::format_key_val(&key, val, self.with_counts))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_count_lines_simple() {
        let reader = BufReader::new(Cursor::new("hello\nhello\nhi".to_string()));
        let uniq_reader = UniqueReader::new(reader);
        let outputs: Vec<_> = uniq_reader
            .into_line_counts()
            .include_counts()
            .into_lines()
            .collect();
        assert_eq!(outputs, vec!["   2 hello\n", "   1 hi\n"]);
    }
    #[test]
    fn test_unique_lines() {
        let reader = BufReader::new(Cursor::new("hello\nhello\nhi".to_string()));
        let uniq_reader = UniqueReader::new(reader);
        let outputs: Vec<_> = uniq_reader
            .unique()
            .into_line_counts()
            .into_lines()
            .collect();
        assert_eq!(outputs, vec!["hi\n"]);
    }

    #[test]
    fn test_dedup_adjacent_lines() {
        let reader = BufReader::new(Cursor::new("hello\nhello".to_string()));
        let uniq_reader = UniqueReader::new(reader);
        let deduped_lines: Vec<_> = uniq_reader.into_line_counts().into_lines().collect();
        assert_eq!(deduped_lines, vec!["hello\n".to_string()])
    }
    #[test]
    fn test_no_dedup() {
        let reader = BufReader::new(Cursor::new("hello\nworld".to_string()));
        let uniq_reader = UniqueReader::new(reader);
        let deduped_lines: Vec<_> = uniq_reader.into_line_counts().into_lines().collect();
        assert_eq!(
            deduped_lines,
            vec!["hello\n".to_string(), "world\n".to_string()]
        )
    }
    #[test]
    fn test_extra_newline() {
        let reader = BufReader::new(Cursor::new("hello\nworld\r\n\n".to_string()));
        let uniq_reader = UniqueReader::new(reader);
        let deduped_lines: Vec<_> = uniq_reader.into_line_counts().into_lines().collect();
        assert_eq!(
            deduped_lines,
            // TODO: this fails because we aren't clearing out the empty line
            // want to pull the "read,unwrap lines" logic into own struct first
            vec!["hello\n".to_string(), "world\n".to_string()]
        )
    }
    #[test]
    fn test_empty_duplicated() {
        let reader = BufReader::new(Cursor::new("".to_string()));
        let uniq_reader = UniqueReader::new(reader).repeated();
        let duplicate_lines: Vec<_> = uniq_reader.into_line_counts().into_lines().collect();
        assert_eq!(duplicate_lines, Vec::<String>::new());
    }
    #[test]
    fn test_duplicated() {
        let reader = BufReader::new(Cursor::new("hello\nhello".to_string()));
        let uniq_reader = UniqueReader::new(reader).repeated();
        let duplicate_lines: Vec<_> = uniq_reader.into_line_counts().into_lines().collect();
        assert_eq!(duplicate_lines, vec!["hello\n".to_string()]);
    }
    #[test]
    fn test_duplicated_straggler() {
        let reader = BufReader::new(Cursor::new("hello\nhello\nhi".to_string()));
        let uniq_reader = UniqueReader::new(reader).repeated();
        let duplicate_lines: Vec<_> = uniq_reader.into_line_counts().into_lines().collect();
        assert_eq!(duplicate_lines, vec!["hello\n".to_string()]);
    }
    #[test]
    fn test_regular_case_insensitive_lower_first() {
        let reader = BufReader::new(Cursor::new("hello\nHELLO\n".to_string()));
        let ci_reader = UniqueReader::new(reader).case_insensitive();
        let lines: Vec<_> = ci_reader.into_line_counts().into_lines().collect();
        assert_eq!(lines, vec!["hello\n".to_string()]);
    }
    #[test]
    fn test_regular_case_insensitive_upper_first() {
        let reader = BufReader::new(Cursor::new("HELLO\nhello\n".to_string()));
        let ci_reader = UniqueReader::new(reader).case_insensitive();
        let lines: Vec<_> = ci_reader.into_line_counts().into_lines().collect();
        assert_eq!(lines, vec!["HELLO\n".to_string()]);
    }
}
