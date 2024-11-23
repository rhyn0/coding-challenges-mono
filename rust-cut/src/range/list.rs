use super::cut::{CutRange, ListCutStrError, Selector};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct CutList {
    /// Collection of `CtuRange`
    container: Vec<CutRange>,
}

impl CutList {
    pub fn new(ranges: Vec<CutRange>) -> Self {
        Self { container: ranges }
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.container.len()
    }
    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }
}

impl Selector for CutList {
    fn is_selected(&self, field: usize) -> bool {
        self.container.iter().any(|c| c.is_selected(field))
    }
}

impl FromStr for CutList {
    type Err = ListCutStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_results: Result<Vec<_>, _> = s
            .split_whitespace()
            .flat_map(|part| part.split(','))
            .map(CutRange::from_str)
            .collect();
        match from_results {
            Ok(v) => Ok(Self::new(v)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_whitespace() {
        assert_eq!(
            CutList::from_str("1 2").unwrap(),
            CutList::new(vec![CutRange::from(1), CutRange::from(2)])
        );
    }
    #[test]
    fn test_parse_comma() {
        assert_eq!(
            CutList::from_str("1,2").unwrap(),
            CutList::new(vec![CutRange::from(1), CutRange::from(2)])
        );
    }
    #[test]
    fn test_parse_mixed() {
        assert_eq!(
            CutList::from_str("1 2,3").unwrap(),
            CutList::new(vec![
                CutRange::from(1),
                CutRange::from(2),
                CutRange::from(3)
            ])
        );
    }
    #[test]
    fn test_illegal_values() {
        assert!(CutList::from_str("a 1,2").is_err(),);
        assert!(CutList::from_str("1 a,2").is_err(),);
        assert!(CutList::from_str("1 2,a").is_err(),);
    }
}
