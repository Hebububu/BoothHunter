use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

use super::models::*;
use super::parser;
use crate::error::{AppError, AppResult};

pub struct BoothClient {
    client: Client,
    last_request: Arc<Mutex<Instant>>,
}

impl BoothClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(15))
            .redirect(reqwest::redirect::Policy::limited(5))
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            last_request: Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1))),
        }
    }

    /// Rate limiting: wait at least 1s between requests
    async fn rate_limit(&self) {
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        if elapsed < Duration::from_millis(1000) {
            tokio::time::sleep(Duration::from_millis(1000) - elapsed).await;
        }
        *last = Instant::now();
    }

    /// Search Booth.pm via HTML scraping (no JSON search API available)
    pub async fn search(&self, params: &SearchParams) -> AppResult<SearchResult> {
        self.rate_limit().await;

        let page = params.page.unwrap_or(1).min(10_000);

        // Validate category length
        if let Some(ref cat) = params.category {
            if cat.len() > 100 {
                return Err(AppError::ParseError("Category too long".to_string()));
            }
        }

        // Validate price bounds
        if let Some(min) = params.price_min {
            if min > 999_999_999 {
                return Err(AppError::ParseError("Invalid price_min".to_string()));
            }
        }
        if let Some(max) = params.price_max {
            if max > 999_999_999 {
                return Err(AppError::ParseError("Invalid price_max".to_string()));
            }
        }
        let keyword_empty = params.keyword.trim().is_empty();

        // When a category is specified, use /ja/browse/{category}
        // When keyword is empty with category, omit q= parameter for pure browsing
        // Otherwise use /ja/items?q=...
        let mut url = if let Some(ref category) = params.category {
            if keyword_empty {
                format!(
                    "https://booth.pm/ja/browse/{}?page={}",
                    urlencoded(category),
                    page
                )
            } else {
                format!(
                    "https://booth.pm/ja/browse/{}?q={}&page={}",
                    urlencoded(category),
                    urlencoded(&params.keyword),
                    page
                )
            }
        } else {
            format!(
                "https://booth.pm/ja/items?q={}&page={}",
                urlencoded(&params.keyword),
                page
            )
        };

        if let Some(ref sort) = params.sort {
            // Allowlist to prevent query parameter injection
            let valid_sorts = ["new", "popular", "price_asc", "price_desc"];
            if valid_sorts.contains(&sort.as_str()) {
                url.push_str(&format!("&sort={}", sort));
            }
        }

        if params.only_free == Some(true) {
            url.push_str("&max_price=0");
        } else {
            if let Some(min) = params.price_min {
                url.push_str(&format!("&min_price={}", min));
            }
            if let Some(max) = params.price_max {
                url.push_str(&format!("&max_price={}", max));
            }
        }

        let resp = self.client.get(&url).send().await?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(AppError::RateLimited);
        }

        if !resp.status().is_success() {
            return Err(AppError::NotFound(format!(
                "Search returned {}",
                resp.status()
            )));
        }

        let html = resp.text().await?;
        let (items, total_count) = parser::parse_search_html(&html);

        Ok(SearchResult {
            items,
            total_count,
            current_page: page,
        })
    }

    /// Get a single Booth item by ID - try JSON first, then HTML
    pub async fn get_item(&self, item_id: u64) -> AppResult<BoothItem> {
        self.rate_limit().await;

        // Try JSON API first (works for individual items)
        match self.get_item_json(item_id).await {
            Ok(item) => return Ok(item),
            Err(e) => {
                log::warn!("JSON item fetch failed, falling back to HTML: {}", e);
            }
        }

        // Fallback to HTML
        self.get_item_html(item_id).await
    }

    async fn get_item_json(&self, item_id: u64) -> AppResult<BoothItem> {
        let url = format!("https://booth.pm/ja/items/{}.json", item_id);

        let resp = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(AppError::RateLimited);
        }

        if !resp.status().is_success() {
            return Err(AppError::NotFound(format!("Item {} not found", item_id)));
        }

        let body = resp.text().await?;
        let detail: BoothJsonItemDetail =
            serde_json::from_str(&body).map_err(|e| AppError::ParseError(e.to_string()))?;

        detail
            .into_booth_item()
            .ok_or_else(|| AppError::ParseError("Failed to parse item detail".to_string()))
    }

    async fn get_item_html(&self, item_id: u64) -> AppResult<BoothItem> {
        let url = format!("https://booth.pm/ja/items/{}", item_id);
        let resp = self.client.get(&url).send().await?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(AppError::RateLimited);
        }

        let html = resp.text().await?;
        parser::parse_item_detail_html(&html, item_id)
            .ok_or_else(|| AppError::NotFound(format!("Item {} not found in HTML", item_id)))
    }
}

fn urlencoded(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                c.to_string()
            } else {
                let mut buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut buf);
                encoded.bytes().map(|b| format!("%{:02X}", b)).collect()
            }
        })
        .collect()
}
