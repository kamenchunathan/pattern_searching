fn main() {
    let text = "ACCACGCGTATGCTCTCCG";
    let pattern = "ACCA";
    if let Some(matches) = match_exact_naive(pattern, text) {
        print_matches(text, matches);
    }
}

/// <summary>
/// Exact matching using the naive method,
/// Aligns the left of the pattern (p) to the left ot the text (T) and compares
/// the characters of p and T left to right until two unequal characters are
/// found or until p is exhausted
/// p is then shifted one step to the right and comparisons are restarted from
/// the left end of p, repeating until the end of p shifts past the end of T
/// </summary>
#[allow(non_snake_case)]
fn match_exact_naive(P: &str, T: &str) -> Option<Vec<(usize, usize)>> {
    let mut match_start_indices = Vec::new();
    'text_loop: for i in 0..T.len() {
        for j in 0..P.len() {
            if i + P.len() > T.len() {
                break 'text_loop;
            }

            if P.chars().nth(j) != T.chars().nth(i + j) {
                break;
            }

            if j == P.len() - 1 {
                // println!("Found it {}", &T[i..=i + j]);
                match_start_indices.push((i, i + j + 1));
            }
        }
    }
    Some(match_start_indices)
}

// NOTE: Assumes no overlap
fn print_matches(text: &str, pos: Vec<(usize, usize)>) {
    if pos.len() <= 0 || text.len() <= 0 { return; }

    let mut t = String::from(text);
    for (i, j) in pos {
        t = format!("{}\x1b[93m{}\x1b[0m{}", t[0..i].to_string(), t[i..j].to_string(), t[j..].to_string());
    }
    println!("{}", t);
}