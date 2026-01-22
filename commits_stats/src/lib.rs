use std::collections::HashMap;
use chrono::{DateTime, Datelike, Utc};
use json::JsonValue;

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut stats = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(author) = commit["author"]["login"].as_str() {
                *stats.entry(author.to_string()).or_insert(0) += 1;
            }
        }
    }
    stats
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut stats = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
                if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                    let date_utc = date.with_timezone(&Utc);
                    let iso_week = date_utc.iso_week();
                    
                    let week_key = format!("{}-W{}", iso_week.year(), iso_week.week());
                    
                    *stats.entry(week_key).or_insert(0) += 1;
                }
            }
        }
    }
    stats
}