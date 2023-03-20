use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: <program_name> <string1> <string2> ...");
        std::process::exit(1);
    }

    let ac = AhoCorasick::new(&args);

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let mut file = File::open(&path)
                .with_context(|| format!("Failed to open file: {:?}", path))?;

            let mut contents = String::new();
            let Ok(_) = file.read_to_string(&mut contents) else {
                continue;
            };

            if contains_all_patterns(&ac, &contents) {
                println!("Found all patterns in {:?}", path);
            }
        }
    }

    Ok(())
}

fn contains_all_patterns(ac: &AhoCorasick, contents: &str) -> bool {
    let mut found = vec![false; ac.pattern_count()];

    for m in ac.find_iter(contents) {
        found[m.pattern()] = true;
    }

    found.iter().all(|&x| x)
}
