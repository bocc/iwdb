use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(super) fn add_words<P>(path: P) -> Result<Vec<String>, std::io::Error>
where
    P: AsRef<Path>,
{
    let input = File::open(path)?;

    let input = io::BufReader::new(input);

    let lines: Vec<_> = input.lines().filter_map(Result::ok).collect();

    Ok(lines)
}
