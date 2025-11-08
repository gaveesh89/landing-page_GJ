use serde::Deserialize;
use pulldown_cmark::{Parser, html};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub content: String,
}

/// Parse frontmatter from markdown content
/// Returns (title, date, excerpt, body) tuple
pub fn parse_frontmatter(content: &str) -> (String, String, String, String) {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    
    if parts.len() < 3 {
        // No frontmatter found, return empty metadata
        return ("Untitled".to_string(), "".to_string(), "".to_string(), content.to_string());
    }
    
    let frontmatter = parts[1].trim();
    let body = parts[2].trim();
    
    let mut title = "Untitled".to_string();
    let mut date = "".to_string();
    let mut excerpt = "".to_string();
    
    // Parse YAML-like frontmatter
    for line in frontmatter.lines() {
        let line = line.trim();
        if line.starts_with("title:") {
            if let Some(title_value) = line.strip_prefix("title:") {
                title = title_value.trim().trim_matches('"').to_string();
            }
        } else if line.starts_with("date:") {
            if let Some(date_value) = line.strip_prefix("date:") {
                date = date_value.trim().trim_matches('"').to_string();
            }
        } else if line.starts_with("excerpt:") {
            if let Some(excerpt_value) = line.strip_prefix("excerpt:") {
                excerpt = excerpt_value.trim().trim_matches('"').to_string();
            }
        }
    }
    
    (title, date, excerpt, body.to_string())
}

/// Convert markdown content to HTML
pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Parse a single post from slug and raw markdown content
pub fn parse_post(slug: &str, raw_content: &str) -> Post {
    let (title, date, excerpt, markdown_body) = parse_frontmatter(raw_content);
    let content = markdown_to_html(&markdown_body);
    
    Post {
        slug: slug.to_string(),
        title,
        date,
        excerpt,
        content,
    }
}

/// Get all blog posts, sorted by date descending
pub fn get_all_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    
    // Load all markdown files using include_str!
    let hello_world_content = include_str!("../content/posts/hello-world.md");
    let getting_started_content = include_str!("../content/posts/getting-started.md");
    let rust_tips_content = include_str!("../content/posts/rust-tips.md");
    let my_new_topic_content = include_str!("../content/posts/What is an atomic increment.md");
    
    // Parse each post
    posts.push(parse_post("hello-world", hello_world_content));
    posts.push(parse_post("getting-started", getting_started_content));
    posts.push(parse_post("rust-tips", rust_tips_content));
    posts.push(parse_post("my-new-topic", my_new_topic_content));
    
    // Sort by date descending (newest first)
    posts.sort_by(|a, b| {
        // Parse dates for comparison
        let date_a = chrono::NaiveDate::parse_from_str(&a.date, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let date_b = chrono::NaiveDate::parse_from_str(&b.date, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        
        date_b.cmp(&date_a) // Descending order (newest first)
    });
    
    posts
}

/// Find a specific post by slug
pub fn get_post_by_slug(slug: &str) -> Option<Post> {
    get_all_posts().into_iter().find(|post| post.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: "Test Post"
date: "2024-11-08"
excerpt: "This is a test excerpt"
---

# Hello World

This is a test post."#;

        let (title, date, excerpt, body) = parse_frontmatter(content);
        assert_eq!(title, "Test Post");
        assert_eq!(date, "2024-11-08");
        assert_eq!(excerpt, "This is a test excerpt");
        assert!(body.contains("# Hello World"));
    }

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_parse_post() {
        let content = r#"---
title: "Test Post"
date: "2024-11-08"
---

# Hello World"#;

        let post = parse_post("test-slug", content);
        assert_eq!(post.slug, "test-slug");
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.date, "2024-11-08");
        assert!(post.content.contains("<h1>"));
    }

    #[test]
    fn test_get_all_posts() {
        let posts = get_all_posts();
        assert_eq!(posts.len(), 4);
        
        // Check that posts are loaded
        let slugs: Vec<&str> = posts.iter().map(|p| p.slug.as_str()).collect();
        assert!(slugs.contains(&"hello-world"));
        assert!(slugs.contains(&"getting-started"));
        assert!(slugs.contains(&"rust-tips"));
        assert!(slugs.contains(&"my-new-topic"));
    }

    #[test]
    fn test_get_post_by_slug() {
        let post = get_post_by_slug("hello-world");
        assert!(post.is_some());
        
        let post = post.unwrap();
        assert_eq!(post.slug, "hello-world");
        assert!(post.title.contains("Hello World"));
        
        // Test non-existent post
        let missing_post = get_post_by_slug("nonexistent");
        assert!(missing_post.is_none());
    }
}