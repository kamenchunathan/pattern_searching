/// Exact matching using the naive method,
///
/// Aligns the left of the pattern (P) to the left ot the text (T) and compares
/// the characters of P and T left to right until two unequal characters are
/// found or until P is exhausted
/// P is then shifted one step to the right and comparisons are restarted from
/// the left end of P, repeating until the end of P shifts past the end of T
#[allow(non_snake_case)]
pub fn match_exact_naive(pattern: &str, text: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return vec![];
    }

    // A vector of the matches
    // A match is recorded as the index of the letter in the text where the match
    // occured and the length of the characters that match
    let mut matches = Vec::new();

    'text_loop: for i in 0..text.len() {
        for j in 0..pattern.len() {
            // the end of P has moved past the end of T
            if i + pattern.len() > text.len() {
                break 'text_loop;
            }

            // unequal characters
            if pattern.chars().nth(j) != text.chars().nth(i + j) {
                break;
            }

            // A Match has been found: end of the pattern without breaking the loop
            if j == pattern.len() - 1 {
                matches.push(i);
            } else {
                println!("j: {}, pattern.len(): {}", j, pattern.len());
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use crate::naive_self::match_exact_naive;

    #[test]
    fn empty_pattern() {
        assert_eq!(match_exact_naive("", "Hello World",), []);
    }

    #[test]
    fn empty_text() {
        assert_eq!(match_exact_naive("nothing", "",), []);
    }

    #[test]
    fn empty_pattern_and_text() {
        assert_eq!(match_exact_naive("", "",), []);
    }

    #[test]
    fn match_first_character() {
        assert_eq!(match_exact_naive("H", "Hello World",), [0]);
    }

    #[test]
    fn match_last_character() {
        assert_eq!(match_exact_naive("d", "Hello World",), [10]);
    }

    #[test]
    fn match_single_character() {
        assert_eq!(match_exact_naive("e", "Hello World",), [1]);
    }

    #[test]
    fn multiple_occurences_of_single_char() {
        assert_eq!(match_exact_naive("o", "Hello World",), [4, 7]);
    }

    #[test]
    fn multiple_occurences_longer_pattern() {
        assert_eq!(
            match_exact_naive("ell", "Hello to a yellow hell",),
            [1, 12, 19]
        );
    }

    #[test]
    fn consequtive_occurences_of_single_char() {
        assert_eq!(match_exact_naive("l", "Hello World",), [2, 3, 9]);
    }

    #[test]
    fn overlapping_sequences() {
        assert_eq!(match_exact_naive("pip", "pipip",), [0, 2]);
    }
}
