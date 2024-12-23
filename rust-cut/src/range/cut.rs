use log::debug;
use std::{
    ops::{RangeFrom, RangeInclusive},
    str::FromStr,
};
use thiserror::Error as TIError;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum CutRange {
    Single(usize),
    OpenEnd(RangeFrom<usize>),
    Closed(RangeInclusive<usize>),
}

pub trait Selector {
    fn is_selected(&self, field: usize) -> bool;
}

impl Selector for CutRange {
    fn is_selected(&self, field: usize) -> bool {
        match &self {
            Self::Single(x) => *x == field,
            Self::Closed(rg) => rg.contains(&field),
            Self::OpenEnd(rg) => rg.contains(&field),
        }
    }
}

#[derive(Debug, TIError, PartialEq, Eq)]
pub enum ListCutStrError {
    #[error("values may not include zero")]
    ListMayNotIncludeZero,
    #[error("illegal list value")]
    IllegalListValue,
}

/// Helpers for testing
impl From<usize> for CutRange {
    /// This could break other things if we pass in `0`
    fn from(value: usize) -> Self {
        Self::Single(value)
    }
}
impl From<RangeFrom<usize>> for CutRange {
    /// This could break other things if we pass in `0..`
    fn from(value: RangeFrom<usize>) -> Self {
        Self::OpenEnd(value)
    }
}
impl From<RangeInclusive<usize>> for CutRange {
    /// This could break other things if we pass in `0..=1`
    fn from(value: RangeInclusive<usize>) -> Self {
        Self::Closed(value)
    }
}

/// Important Argument parsing logic
impl FromStr for CutRange {
    type Err = ListCutStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // check that all values are valid. Numbers and `-`
        debug!("Started parsing Range from {s}");
        let hyphen_count = s.chars().fold(0, |acc, c| acc + usize::from(c == '-'));
        debug!("Input has {hyphen_count} hyphens");
        if !s.chars().all(|c| c.is_ascii_digit() || c == '-') || hyphen_count > 1 {
            return Err(ListCutStrError::IllegalListValue);
        }
        if hyphen_count == 0 {
            // it is a singular field value
            match s.parse::<usize>() {
                Ok(0) => Err(ListCutStrError::ListMayNotIncludeZero),
                Ok(x) => Ok(Self::Single(x)),
                Err(_) => Err(ListCutStrError::IllegalListValue),
            }
        } else if s.starts_with('-') {
            debug!("Determined range is implied first field inclusive.");
            // we end up with at least one "number" after the split
            // we can convert `front` to usize and then figure out with other info if we need to unwrap `back`
            match s
                .split('-')
                .nth(1)
                .expect("To be a string after splitting.")
                .parse::<usize>()
            {
                Ok(0) => Err(ListCutStrError::ListMayNotIncludeZero),
                Ok(x) => Ok(Self::Closed(1..=x)),
                // not a number
                Err(_) => Err(ListCutStrError::IllegalListValue),
            }
        } else {
            let mut parts = s.split('-');
            let front = match parts
                .next()
                .expect("To be a string after splitting.")
                .parse::<usize>()
            {
                Ok(0) => return Err(ListCutStrError::ListMayNotIncludeZero),
                Ok(x) => x,
                // not a number
                Err(_) => return Err(ListCutStrError::IllegalListValue),
            };
            debug!("Successfully parsed {front} as first field");
            let back = parts
                .next()
                .expect("The end of the string after split on character");
            if back.is_empty() {
                // received a list like `10-`
                Ok(Self::OpenEnd(front..))
            } else {
                // protected by above check
                let back = match back.parse::<usize>() {
                    Ok(0) => return Err(ListCutStrError::ListMayNotIncludeZero),
                    Ok(x) => x,
                    Err(_) => return Err(ListCutStrError::IllegalListValue),
                };
                Ok(Self::Closed(front..=back))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[test]
    fn test_illegal_list_val_errs() {
        assert!(CutRange::from_str("1:2").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("a").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("1-b").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("a-").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("1--2").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("--2").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("--").is_err_and(|x| x == ListCutStrError::IllegalListValue));
        assert!(CutRange::from_str("-").is_err_and(|x| x == ListCutStrError::IllegalListValue));
    }
    #[test]
    fn test_no_zeroes_allowed() {
        init_logger();
        assert!(CutRange::from_str("0").is_err_and(|x| x == ListCutStrError::ListMayNotIncludeZero));
        assert!(
            CutRange::from_str("0-").is_err_and(|x| x == ListCutStrError::ListMayNotIncludeZero)
        );
        assert!(
            CutRange::from_str("-0").is_err_and(|x| x == ListCutStrError::ListMayNotIncludeZero)
        );
        assert!(
            CutRange::from_str("0-0").is_err_and(|x| x == ListCutStrError::ListMayNotIncludeZero)
        );
    }
    #[test]
    fn test_valid_single() {
        assert!(CutRange::from_str("1").is_ok_and(|x| x == CutRange::Single(1)));
        assert!(CutRange::from_str("10").is_ok_and(|x| x == CutRange::Single(10)));
    }
    #[test]
    fn test_valid_closed() {
        assert!(CutRange::from_str("-2").is_ok_and(|x| x == CutRange::Closed(1..=2)));
        assert!(CutRange::from_str("1-3").is_ok_and(|x| x == CutRange::Closed(1..=3)));
    }
    #[test]
    fn test_valid_open() {
        assert!(CutRange::from_str("1-").is_ok_and(|x| x == CutRange::OpenEnd(1..)));
        assert!(CutRange::from_str("10-").is_ok_and(|x| x == CutRange::OpenEnd(10..)));
    }
    #[test]
    fn test_is_selected() {
        let contained = 2usize;
        assert!(CutRange::from(2).is_selected(contained));
        assert!(CutRange::from(2..).is_selected(contained));
        assert!(CutRange::from(1..=2).is_selected(contained));
    }
}
