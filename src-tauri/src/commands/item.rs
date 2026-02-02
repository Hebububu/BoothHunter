use tauri::State;

use crate::booth::client::BoothClient;
use crate::booth::models::BoothItem;
use crate::error::AppResult;

#[tauri::command]
pub async fn get_booth_item(
    client: State<'_, BoothClient>,
    item_id: u64,
) -> AppResult<BoothItem> {
    client.get_item(item_id).await
}
