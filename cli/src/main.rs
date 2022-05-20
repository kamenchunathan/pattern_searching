use std::env;

use algos_on_strings_trees_sequences::match_exact_naive;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let text = &args[2];

    let indices = match_exact_naive(pattern, text);
    println!("{:?}", indices);

    Ok(())
}
