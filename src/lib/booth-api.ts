import { invoke } from "@tauri-apps/api/core";
import type { BoothItem, SearchParams, SearchResult, FavoriteItem } from "./types";

// ── Search API ─────────────────────────────────────────

export async function searchBooth(params: SearchParams): Promise<SearchResult> {
  return invoke<SearchResult>("search_booth", { params });
}

export async function getBoothItem(itemId: number): Promise<BoothItem> {
  return invoke<BoothItem>("get_booth_item", { itemId });
}

// ── Cache / History ────────────────────────────────────

export async function cacheItems(items: BoothItem[]): Promise<void> {
  return invoke("cache_items", { items });
}

export async function saveSearchHistory(keyword: string): Promise<void> {
  return invoke("save_search_history", { keyword });
}

// ── Favorites ──────────────────────────────────────────

export async function getFavorites(): Promise<FavoriteItem[]> {
  return invoke<FavoriteItem[]>("get_favorites");
}

export async function addFavorite(params: {
  item_id: number;
  name: string;
  price: number;
  thumbnail_url: string | null;
  category_name: string | null;
  shop_name: string | null;
}): Promise<void> {
  return invoke("add_favorite", { params });
}

export async function removeFavorite(itemId: number): Promise<void> {
  return invoke("remove_favorite", { itemId });
}

// ── Popular Avatars ────────────────────────────────────

export interface PopularAvatar {
  id: number;
  name_ja: string;
  name_ko: string;
  item_count: number;
  thumbnail_url: string | null;
  updated_at: string;
  is_default: number;
}

export async function getPopularAvatars(): Promise<PopularAvatar[]> {
  return invoke<PopularAvatar[]>("get_popular_avatars");
}

export async function checkAvatarsNeedUpdate(): Promise<boolean> {
  return invoke<boolean>("check_avatars_need_update");
}

export async function updatePopularAvatar(
  id: number,
  itemCount: number,
  thumbnailUrl: string | null,
): Promise<void> {
  return invoke("update_popular_avatar", { id, itemCount, thumbnailUrl });
}
