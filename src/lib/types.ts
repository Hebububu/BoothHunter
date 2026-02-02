export interface BoothItem {
  id: number;
  name: string;
  description: string | null;
  price: number;
  category_name: string | null;
  shop_name: string | null;
  url: string;
  images: string[];
  tags: string[];
}

export interface SearchParams {
  keyword: string;
  page?: number;
  category?: string;
  sort?: string;
  only_free?: boolean;
  price_min?: number;
  price_max?: number;
}

export interface SearchResult {
  items: BoothItem[];
  total_count: number | null;
  current_page: number;
}

export interface FavoriteItem {
  id: number;
  item_id: number;
  name: string;
  price: number;
  thumbnail_url: string | null;
  category_name: string | null;
  shop_name: string | null;
  added_at: string;
  note: string | null;
}

export interface SearchHistoryEntry {
  id: number;
  keyword: string;
  searched_at: string;
}
