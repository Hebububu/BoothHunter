#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoothItem {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub price: u64,
    pub category_name: Option<String>,
    pub shop_name: Option<String>,
    pub url: String,
    pub images: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchParams {
    pub keyword: String,
    pub page: Option<u32>,
    pub category: Option<String>,
    pub sort: Option<String>,
    pub only_free: Option<bool>,
    pub price_min: Option<u64>,
    pub price_max: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub items: Vec<BoothItem>,
    pub total_count: Option<u64>,
    pub current_page: u32,
}

/// Booth.pm JSON API response structures
#[derive(Debug, Deserialize)]
pub struct BoothJsonSearchResponse {
    pub items: Option<Vec<BoothJsonItem>>,
    pub total_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BoothJsonItem {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<serde_json::Value>,
    pub category: Option<BoothJsonCategory>,
    pub shop: Option<BoothJsonShop>,
    pub url: Option<String>,
    pub images: Option<Vec<BoothJsonImage>>,
    pub tags: Option<Vec<BoothJsonTag>>,
    pub wish_lists_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BoothJsonCategory {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BoothJsonShop {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BoothJsonImage {
    pub original: Option<String>,
    pub resized: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BoothJsonTag {
    pub name: Option<String>,
}

/// Single item detail response
#[derive(Debug, Deserialize)]
pub struct BoothJsonItemDetail {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<serde_json::Value>,
    pub category: Option<BoothJsonCategory>,
    pub shop: Option<BoothJsonShop>,
    pub url: Option<String>,
    pub images: Option<Vec<BoothJsonImage>>,
    pub tags: Option<Vec<BoothJsonTag>>,
    pub wish_lists_count: Option<u64>,
}

impl BoothJsonItem {
    pub fn into_booth_item(self) -> Option<BoothItem> {
        let id = self.id?;
        let name = self.name.unwrap_or_default();
        let price = parse_price(&self.price);
        let url = self.url.unwrap_or_else(|| format!("https://booth.pm/ja/items/{}", id));
        let images: Vec<String> = self
            .images
            .unwrap_or_default()
            .into_iter()
            .filter_map(|img| img.original.or(img.resized))
            .collect();
        let tags: Vec<String> = self
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| t.name)
            .collect();

        Some(BoothItem {
            id,
            name,
            description: self.description,
            price,
            category_name: self.category.and_then(|c| c.name),
            shop_name: self.shop.and_then(|s| s.name),
            url,
            images,
            tags,
        })
    }
}

impl BoothJsonItemDetail {
    pub fn into_booth_item(self) -> Option<BoothItem> {
        let id = self.id?;
        let name = self.name.unwrap_or_default();
        let price = parse_price(&self.price);
        let url = self.url.unwrap_or_else(|| format!("https://booth.pm/ja/items/{}", id));
        let images: Vec<String> = self
            .images
            .unwrap_or_default()
            .into_iter()
            .filter_map(|img| img.original.or(img.resized))
            .collect();
        let tags: Vec<String> = self
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| t.name)
            .collect();

        Some(BoothItem {
            id,
            name,
            description: self.description,
            price,
            category_name: self.category.and_then(|c| c.name),
            shop_name: self.shop.and_then(|s| s.name),
            url,
            images,
            tags,
        })
    }
}

fn parse_price(val: &Option<serde_json::Value>) -> u64 {
    match val {
        Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => {
            let cleaned: String = s
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            cleaned.parse().unwrap_or(0)
        }
        _ => 0,
    }
}
