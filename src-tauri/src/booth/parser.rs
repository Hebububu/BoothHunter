use once_cell::sync::Lazy;
use scraper::{Html, Selector};

use super::models::BoothItem;

// ── Cached selectors (parsed once, reused) ────────────────

static SEARCH_ITEM_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("li.item-card[data-product-id]").expect("invalid selector: SEARCH_ITEM_SEL")
});

static CARD_IMG_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("a.js-thumbnail-image[data-original]").expect("invalid selector: CARD_IMG_SEL")
});

static CARD_SHOP_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-card__shop-name").expect("invalid selector: CARD_SHOP_SEL")
});

static CARD_CAT_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-card__category-anchor").expect("invalid selector: CARD_CAT_SEL")
});

static CARD_TITLE_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-card__title-anchor--multiline, .item-card__title a")
        .expect("invalid selector: CARD_TITLE_SEL")
});

static DETAIL_TITLE_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".u-tpg-title2, h2.u-tpg-title2, .item-name h1, [data-product-name]")
        .expect("invalid selector: DETAIL_TITLE_SEL")
});

static DETAIL_PRICE_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-price .u-tpg-body1, .price, .u-tpg-title2-price")
        .expect("invalid selector: DETAIL_PRICE_SEL")
});

static DETAIL_DESC_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".u-mb-400 .u-tpg-body1, .item-description, .description")
        .expect("invalid selector: DETAIL_DESC_SEL")
});

static DETAIL_IMG_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-gallery img, .slick-slide img, .market-item-detail-item-image img")
        .expect("invalid selector: DETAIL_IMG_SEL")
});

static DETAIL_SHOP_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".shop-name, .shop-name-mini a, .u-d-ib a")
        .expect("invalid selector: DETAIL_SHOP_SEL")
});

static DETAIL_TAG_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-tag a, a.tag, .item-info-tag a")
        .expect("invalid selector: DETAIL_TAG_SEL")
});

static DETAIL_CAT_SEL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".item-category a, .category-name a")
        .expect("invalid selector: DETAIL_CAT_SEL")
});

// ── Search results parsing ────────────────────────────────

/// Parse Booth.pm search results HTML page.
/// Returns (items, total_count).
pub fn parse_search_html(html: &str) -> (Vec<BoothItem>, Option<u64>) {
    let document = Html::parse_document(html);

    let mut items = Vec::new();
    for element in document.select(&SEARCH_ITEM_SEL) {
        if let Some(item) = parse_item_card(&element) {
            items.push(item);
        }
    }

    let total_count = parse_total_count(&document);
    (items, total_count)
}

fn parse_item_card(element: &scraper::ElementRef) -> Option<BoothItem> {
    let el = element.value();

    let id: u64 = el.attr("data-product-id")?.parse().ok()?;
    let name = el
        .attr("data-product-name")
        .unwrap_or("")
        .to_string();
    let price: u64 = el
        .attr("data-product-price")
        .and_then(|p| p.parse().ok())
        .unwrap_or(0);

    let images: Vec<String> = element
        .select(&CARD_IMG_SEL)
        .filter_map(|img| img.value().attr("data-original").map(|s| s.to_string()))
        .collect();

    let shop_name = element
        .select(&CARD_SHOP_SEL)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty());

    let category_name = element
        .select(&CARD_CAT_SEL)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty());

    let final_name = if name.is_empty() {
        element
            .select(&CARD_TITLE_SEL)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    } else {
        name
    };

    if final_name.is_empty() {
        return None;
    }

    let url = format!("https://booth.pm/ja/items/{}", id);

    Some(BoothItem {
        id,
        name: final_name,
        description: None,
        price,
        category_name,
        shop_name,
        url,
        images,
        tags: Vec::new(),
    })
}

fn parse_total_count(document: &Html) -> Option<u64> {
    let selectors = [
        ".u-tpg-caption1",
        ".search-result-count",
        ".u-tpg-body2",
        "title",
    ];
    for sel_str in &selectors {
        if let Ok(sel) = Selector::parse(sel_str) {
            for element in document.select(&sel) {
                let text = element.text().collect::<String>();
                if let Some(count) = extract_count_from_text(&text) {
                    return Some(count);
                }
            }
        }
    }
    None
}

fn extract_count_from_text(text: &str) -> Option<u64> {
    let mut num_start = None;
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_ascii_digit() {
            if num_start.is_none() {
                num_start = Some(i);
            }
        } else if chars[i] == ',' && num_start.is_some() {
            // Allow commas in numbers
        } else if (chars[i] == '件' || chars[i] == '点') && num_start.is_some() {
            let num_str: String = chars[num_start.unwrap()..i]
                .iter()
                .filter(|c| c.is_ascii_digit())
                .collect();
            if let Ok(n) = num_str.parse::<u64>() {
                if n > 0 {
                    return Some(n);
                }
            }
            num_start = None;
        } else {
            num_start = None;
        }
    }
    None
}

// ── Item detail parsing ───────────────────────────────────

/// Parse a single Booth.pm item detail page HTML
pub fn parse_item_detail_html(html: &str, item_id: u64) -> Option<BoothItem> {
    let document = Html::parse_document(html);

    let name = document
        .select(&DETAIL_TITLE_SEL)
        .next()
        .map(|el| {
            el.value()
                .attr("data-product-name")
                .map(|s| s.to_string())
                .unwrap_or_else(|| el.text().collect::<String>().trim().to_string())
        })
        .unwrap_or_default();

    if name.is_empty() {
        return None;
    }

    let price = document
        .select(&DETAIL_PRICE_SEL)
        .next()
        .map(|el| {
            let text = el.text().collect::<String>();
            text.replace(['¥', ',', '￥', ' ', '\u{00a0}'], "")
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .unwrap_or(0);

    let description = document
        .select(&DETAIL_DESC_SEL)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty());

    let images: Vec<String> = document
        .select(&DETAIL_IMG_SEL)
        .filter_map(|img| {
            img.value()
                .attr("data-origin")
                .or_else(|| img.value().attr("data-original"))
                .or_else(|| img.value().attr("src"))
                .map(|s| s.to_string())
        })
        .collect();

    let shop_name = document
        .select(&DETAIL_SHOP_SEL)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty());

    let tags: Vec<String> = document
        .select(&DETAIL_TAG_SEL)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let category_name = document
        .select(&DETAIL_CAT_SEL)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty());

    Some(BoothItem {
        id: item_id,
        name,
        description,
        price,
        category_name,
        shop_name,
        url: format!("https://booth.pm/ja/items/{}", item_id),
        images,
        tags,
    })
}
