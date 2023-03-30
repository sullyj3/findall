use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;
use std::collections::HashMap;

fn usage() -> ! {
    eprintln!("Usage: {} <space separated patterns> <path>", env::args().next().unwrap());
    std::process::exit(1);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = match args.len() {
        0 => usage(),
        1 => ".".to_string(),
        2 => args[1].to_string(),
        _ => usage(),
    };

    // we have one or two arguments
    // interpret the first argument as a space separated list of patterns,
    let patterns: Vec<String> = args[0].split(' ').map(|s| s.to_string()).collect();
    let ac = AhoCorasick::new(&patterns);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let mut file = File::open(&path)
                .with_context(|| format!("Failed to open file: {:?}", path))?;

            let mut contents = String::new();
            let Ok(_) = file.read_to_string(&mut contents) else {
                continue;
            };


            let counts = match_counts(&ac, &patterns, &contents);
            if counts.values().all(|&x| x > 0) {
                println!("{}", path.display());
                for pattern in &patterns {
                    // we iterate the pattern vec rather than the counts map directly
                    // so as to preserve the order of the patterns
                    let count = counts.get(pattern).unwrap();
                    println!("    {}: {}", pattern, count);
                }
            }
        }
    }

    Ok(())
}

fn match_counts(ac: &AhoCorasick, patterns: &[String], contents: &str) -> HashMap<String, i32> {
    let mut counter: HashMap<String, i32> = 
        HashMap::from_iter(patterns.iter().map(|p| (p.clone(), 0)));

    for m in ac.find_iter(contents) {
        counter.entry(patterns[m.pattern()].clone()).and_modify(|c| *c += 1);
    }

    counter
}
