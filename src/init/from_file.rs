use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn add_words<P>(words: &mut HashSet<String>, path: P) -> std::io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut inserted: usize = 0;

    let input = File::open(path)?;

    let input = io::BufReader::new(input);

    for line in input.lines() {
        if let Ok(l) = line {
            words.insert(l.trim().to_string());
            inserted = inserted + 1;
        }
    }

    Ok(inserted)
}
