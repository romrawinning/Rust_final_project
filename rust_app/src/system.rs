use std::fs;
use std::io;

pub fn read_lines(path: &str) -> Result<Vec<String>, String> {
    fs::read_to_string(path)
        .map(|s| s.lines().map(|line| line.to_string()).collect())
        .map_err(|e| format!("Cannot open '{}' : {}", path, e))
}

pub fn filter_lines(lines: &[String], keyword: &str) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.contains(keyword))
        .cloned()
        .collect()
}

pub fn count_matches(lines: &[String], keyword: &str) -> usize {
    lines.iter().filter(|line| line.contains(keyword)).count()
}
