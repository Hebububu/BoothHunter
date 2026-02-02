use tauri::State;

use crate::booth::client::BoothClient;
use crate::booth::models::{SearchParams, SearchResult};
use crate::error::AppResult;

#[tauri::command]
pub async fn search_booth(
    client: State<'_, BoothClient>,
    params: SearchParams,
) -> AppResult<SearchResult> {
    log::info!("search_booth called with keyword='{}', page={:?}", params.keyword, params.page);
    let result = client.search(&params).await;
    match &result {
        Ok(r) => log::info!("search_booth returned {} items, total_count={:?}", r.items.len(), r.total_count),
        Err(e) => log::error!("search_booth error: {}", e),
    }
    result
}
