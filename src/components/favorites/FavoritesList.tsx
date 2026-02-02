import { Link } from "react-router-dom";
import { Heart, Trash2 } from "lucide-react";
import { useFavorites } from "../../hooks/useFavorites";
import { UI_TEXT } from "../../lib/constants";

export default function FavoritesList() {
  const { favorites, isLoading, removeFavorite } = useFavorites();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-20">
        <div className="w-8 h-8 border-3 border-indigo-600 border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  if (favorites.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-gray-400">
        <Heart className="w-16 h-16 mb-4" />
        <p className="text-lg">{UI_TEXT.favorites.empty}</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
      {favorites.map((fav) => (
        <div
          key={fav.id}
          className="bg-white rounded-lg border border-gray-200 overflow-hidden hover:shadow-md transition-shadow group"
        >
          <Link to={`/item/${fav.item_id}`} className="block">
            <div className="aspect-square bg-gray-100 relative overflow-hidden">
              {fav.thumbnail_url ? (
                <img
                  src={fav.thumbnail_url}
                  alt={fav.name}
                  className="w-full h-full object-cover group-hover:scale-105 transition-transform"
                  loading="lazy"
                />
              ) : (
                <div className="w-full h-full flex items-center justify-center text-gray-400 text-sm">
                  No Image
                </div>
              )}
            </div>
          </Link>
          <div className="p-3">
            <div className="flex items-start justify-between gap-1">
              <Link
                to={`/item/${fav.item_id}`}
                className="text-sm font-medium text-gray-900 line-clamp-2 hover:text-indigo-600 flex-1"
              >
                {fav.name}
              </Link>
              <button
                onClick={() => removeFavorite(fav.item_id).catch((e) => console.error("Remove failed:", e))}
                className="p-1 text-gray-300 hover:text-red-500 transition-colors shrink-0"
                title="즐겨찾기 제거"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
            <div className="mt-2 flex items-center justify-between">
              <span
                className={`text-sm font-bold ${fav.price === 0 ? "text-green-600" : "text-gray-900"}`}
              >
                {fav.price === 0
                  ? UI_TEXT.item.free
                  : `¥${fav.price.toLocaleString()}`}
              </span>
              {fav.shop_name && (
                <span className="text-xs text-gray-500 truncate max-w-[120px]">
                  {fav.shop_name}
                </span>
              )}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
