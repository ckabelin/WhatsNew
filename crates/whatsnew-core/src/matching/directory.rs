use serde::{Deserialize, Serialize};

const FEED_DIRECTORY_JSON: &str = include_str!("../../assets/feed_directory.json");

/// A single feed in the curated directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryFeed {
    pub title: String,
    pub url: String,
    pub site: String,
}

/// A category of the curated directory: a set of feeds plus the keywords used to
/// match free-text topics against it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryCategory {
    pub category: String,
    pub keywords: Vec<String>,
    pub feeds: Vec<DirectoryFeed>,
}

/// Loads the curated feed directory bundled with the application.
pub fn load_directory() -> Vec<DirectoryCategory> {
    serde_json::from_str(FEED_DIRECTORY_JSON)
        .expect("bundled feed_directory.json must be valid JSON")
}

/// Matches a free-text topic (e.g. "Rust programming") against the curated
/// directory by keyword overlap and returns feeds from the highest-scoring
/// categor(ies). Returns an empty vec if no category's keywords overlap with the
/// topic's words at all - callers should fall back to autodiscovery/manual entry
/// in that case.
pub fn match_topic<'a>(directory: &'a [DirectoryCategory], topic: &str) -> Vec<&'a DirectoryFeed> {
    let words: Vec<String> = topic
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .collect();

    if words.is_empty() {
        return Vec::new();
    }

    let mut scored: Vec<(usize, &DirectoryCategory)> = directory
        .iter()
        .map(|category| {
            let score = category
                .keywords
                .iter()
                .filter(|kw| words.iter().any(|w| w == &kw.to_lowercase()))
                .count();
            (score, category)
        })
        .filter(|(score, _)| *score > 0)
        .collect();

    scored.sort_by_key(|(score, _)| std::cmp::Reverse(*score));

    let top_score = match scored.first() {
        Some((score, _)) => *score,
        None => return Vec::new(),
    };

    scored
        .into_iter()
        .filter(|(score, _)| *score == top_score)
        .flat_map(|(_, category)| category.feeds.iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_bundled_directory() {
        let directory = load_directory();
        assert!(!directory.is_empty());
        for category in &directory {
            assert!(
                !category.feeds.is_empty(),
                "{} has no feeds",
                category.category
            );
            assert!(
                !category.keywords.is_empty(),
                "{} has no keywords",
                category.category
            );
        }
    }

    #[test]
    fn matches_programming_topic_to_programming_category() {
        let directory = load_directory();
        let feeds = match_topic(&directory, "Rust programming");
        assert!(!feeds.is_empty());
        assert!(feeds
            .iter()
            .any(|f| f.title.contains("Hacker News") || f.title.contains("Lobsters")));
    }

    #[test]
    fn unmatched_topic_returns_empty() {
        let directory = load_directory();
        let feeds = match_topic(&directory, "xyzzy plugh frobnicate");
        assert!(feeds.is_empty());
    }
}
