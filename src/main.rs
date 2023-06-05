use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use walkdir::WalkDir;
use std::collections::HashMap;
use colored::Colorize;

fn usage() -> ! {
    eprintln!("Usage: {} <space separated patterns> <path>", env::args().next().unwrap());
    std::process::exit(1);
}

enum Mode {
    Default,
    FilenameOnly,
}

struct Invocation {
    mode: Mode,
    patterns: Vec<String>,
    path: String,
}

impl Default for Invocation {
    fn default() -> Self {
        let default_path: String = ".".to_string();
        Invocation {
            mode: Mode::Default,
            patterns: Vec::new(),
            path: default_path,
        }
    }
}

impl Invocation {
    fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    fn with_patterns(mut self, patterns: Vec<String>) -> Self {
        self.patterns = patterns;
        self
    }

    fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
}

fn split_patterns(patterns: &str) -> Vec<String> {
    patterns.split(' ').map(|s| s.to_string()).collect()
}

fn parse_args() -> Option<Invocation> {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg_strs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match arg_strs[..] {
        [] => None,
        ["-l", patterns] => Some(
            Invocation::default()
                .with_mode(Mode::FilenameOnly)
                .with_patterns(split_patterns(patterns))
            ),
        ["-l", patterns, path] => Some(
            Invocation::default()
                .with_mode(Mode::FilenameOnly)
                .with_patterns(split_patterns(patterns))
                .with_path(path.to_string())
            ),
        [patterns] => Some(
            Invocation::default()
                .with_patterns(split_patterns(patterns))
            ),
        [patterns, path] => Some(
            Invocation::default()
                .with_patterns(split_patterns(patterns))
                .with_path(path.to_string())
            ),
        _ => None,
    }
}

fn main() -> Result<()> {
    let Invocation {mode, patterns, path} = parse_args().unwrap_or_else(|| usage());
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
                let stripped_path: &Path = path.strip_prefix("./").unwrap_or(path);
                println!("{}", stripped_path.display().to_string().red());

                if let Mode::Default = mode {
                    let match_counts_string = patterns.iter().map(|pattern| {
                        let count = counts.get(pattern).unwrap();
                        format!("{}: {}", pattern.blue(), count)
                    }).collect::<Vec<String>>()
                      .join(", ");
                    println!("    {}", match_counts_string);
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
