//! Core logic for `textstat`: counting lines, words, and bytes.
//!
//! The logic lives in a library crate so it can be unit-tested and
//! integration-tested without spawning the binary.

use std::io::{self, Read};

/// Line, word, and byte counts for a piece of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

/// Computes statistics for an in-memory string.
///
/// Lines are counted as newline characters, matching `wc -l`.
/// Words are runs of non-whitespace characters.
pub fn count(text: &str) -> Stats {
    Stats {
        lines: text.matches('\n').count(),
        words: text.split_whitespace().count(),
        bytes: text.len(),
    }
}

/// Reads all input from `reader` as UTF-8 and computes its statistics.
///
/// # Errors
///
/// Returns an error if reading fails or the input is not valid UTF-8.
pub fn count_reader<R: Read>(mut reader: R) -> io::Result<Stats> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    Ok(count(&text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_is_all_zero() {
        assert_eq!(count(""), Stats::default());
    }

    #[test]
    fn counts_lines_words_and_bytes() {
        let stats = count("hello world\nsecond line\n");
        assert_eq!(
            stats,
            Stats {
                lines: 2,
                words: 4,
                bytes: 24
            }
        );
    }
}