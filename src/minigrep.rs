// Rust Experiment: Building a mini grep clone
// Learning about iterators, file I/O, and CLI args

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <file>", args[0]);
        std::process::exit(1);
    }
    
    let pattern = &args[1];
    let file_path = &args[2];
    
    println!("🔍 Searching for '{}' in '{}'\n", pattern, file_path);
    
    match search_in_file(pattern, file_path) {
        Ok(count) => println!("\n✅ Found {} matches", count),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}

fn search_in_file(pattern: &str, file_path: &str) -> Result<usize, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    
    let mut match_count = 0;
    let mut line_number = 0;
    
    for line in reader.lines() {
        line_number += 1;
        let line = line?;
        
        if line.contains(pattern) {
            match_count += 1;
            println!("{}: {}", line_number, highlight_match(&line, pattern));
        }
    }
    
    Ok(match_count)
}

fn highlight_match(line: &str, pattern: &str) -> String {
    line.replace(pattern, &format!("\x1b[31m{}\x1b[0m", pattern))  // Red color
}

// Test the search function
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_highlight_match() {
        let result = highlight_match("hello world", "world");
        assert!(result.contains("\x1b[31mworld\x1b[0m"));
    }
}
