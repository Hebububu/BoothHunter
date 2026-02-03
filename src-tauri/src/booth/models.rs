use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoothItem {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub category_name: Option<String>,
    pub shop_name: Option<String>,
    pub url: String,
    pub images: Vec<String>,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wish_lists_count: Option<i64>,
}
