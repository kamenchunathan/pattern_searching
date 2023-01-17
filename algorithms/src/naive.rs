/// Exact matching using the naive method,
///
/// Aligns the left of the pattern (P) to the left ot the text (T) and compares
/// the characters of P and T left to right until two unequal characters are
/// found or until P is exhausted
/// P is then shifted one step to the right and comparisons are restarted from
/// the left end of P, repeating until the end of P shifts past the end of T
pub fn match_exact(pattern: &str, text: &str) -> Vec<usize> {
    // A vector of the matches
    // A match is recorded as the index of the letter in the text where the match
    // occured and the length of the characters that match
    let mut matches = Vec::new();

    for i in 0..=(usize::saturating_sub(text.len(), pattern.len())) {
        for j in 0..pattern.len() {
            // unequal characters
            if pattern.chars().nth(j) != text.chars().nth(i + j) {
                break;
            }

            // A Match has been found: end of the pattern without breaking the loop
            if j == pattern.len() - 1 {
                matches.push(i);
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pattern() {
        assert_eq!(match_exact("", "Hello World",), []);
    }

    #[test]
    fn empty_text() {
        assert_eq!(match_exact("nothing", "",), []);
    }

    #[test]
    fn empty_pattern_and_text() {
        assert_eq!(match_exact("", "",), []);
    }

    #[test]
    fn match_first_character() {
        assert_eq!(match_exact("H", "Hello World",), [0]);
    }

    #[test]
    fn match_last_character() {
        assert_eq!(match_exact("d", "Hello World",), [10]);
    }

    #[test]
    fn match_single_character() {
        assert_eq!(match_exact("e", "Hello World",), [1]);
    }

    #[test]
    fn multiple_occurences_of_single_char() {
        assert_eq!(match_exact("o", "Hello World",), [4, 7]);
    }

    #[test]
    fn multiple_occurences_longer_pattern() {
        assert_eq!(match_exact("ell", "Hello to a yellow hell",), [1, 12, 19]);
    }
    #[test]
    fn partial_match_of_the_pattern() {
        assert_eq!(match_exact("ell", "Hello, ask for help",), [1]);
    }

    #[test]
    fn consequtive_occurences_of_single_char() {
        assert_eq!(match_exact("l", "Hello World",), [2, 3, 9]);
    }

    #[test]
    fn overlapping_sequences() {
        assert_eq!(match_exact("pip", "pipip",), [0, 2]);
    }
}
