use mysql::prelude::*;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct SearchHistory {
    pub id: i32,
    pub file_path: String,
    pub keyword: String,
    pub match_count: i32,
    pub created_at: String,
}

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

pub fn test_database() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    let pool = mysql::Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;

    let version: Option<String> = conn.query_first("SELECT VERSION()")?;

    println!("Connected to MySQL: {:?}", version);

    Ok(())
}

pub fn save_search_history(
    file_path: &str,
    keyword: &str,
    match_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    let pool = mysql::Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;

    conn.exec_drop(
        "INSERT INTO search_history
         (file_path, keyword, match_count)
         VALUES (?, ?, ?)",
        (file_path, keyword, match_count),
    )?;

    Ok(())
}

pub fn get_search_history(
) -> Result<Vec<SearchHistory>, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    let pool = mysql::Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;

    let history = conn.query_map(
        "SELECT id, file_path, keyword, match_count,
                DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s')
         FROM search_history
         ORDER BY created_at DESC",
        |(id, file_path, keyword, match_count, created_at): (
            i32, String, String, i32, String
        )| {
            SearchHistory {
                id,
                file_path,
                keyword,
                match_count,
                created_at,
            }
        },
    )?;

    Ok(history)
}
