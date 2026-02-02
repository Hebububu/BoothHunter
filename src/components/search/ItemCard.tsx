import { memo } from "react";
import { Link } from "react-router-dom";
import type { BoothItem } from "../../lib/types";
import { UI_TEXT } from "../../lib/constants";
import FavoriteButton from "../favorites/FavoriteButton";

interface Props {
  item: BoothItem;
}

export default memo(function ItemCard({ item }: Props) {
  const thumbnail = item.images[0] || "";
  const priceText =
    item.price === 0 ? UI_TEXT.item.free : `¥${item.price.toLocaleString()}`;

  return (
    <div className="bg-white rounded-lg border border-gray-200 overflow-hidden hover:shadow-md transition-shadow group">
      <Link to={`/item/${item.id}`} className="block">
        <div className="aspect-square bg-gray-100 relative overflow-hidden">
          {thumbnail ? (
            <img
              src={thumbnail}
              alt={item.name}
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
            to={`/item/${item.id}`}
            className="text-sm font-medium text-gray-900 line-clamp-2 hover:text-indigo-600 flex-1"
          >
            {item.name}
          </Link>
          <FavoriteButton item={item} />
        </div>
        <div className="mt-2 flex items-center justify-between">
          <span
            className={`text-sm font-bold ${item.price === 0 ? "text-green-600" : "text-gray-900"}`}
          >
            {priceText}
          </span>
          {item.shop_name && (
            <span className="text-xs text-gray-500 truncate max-w-[120px]">
              {item.shop_name}
            </span>
          )}
        </div>
        {item.category_name && (
          <span className="inline-block mt-1.5 text-xs text-gray-500 bg-gray-100 px-2 py-0.5 rounded">
            {item.category_name}
          </span>
        )}
      </div>
    </div>
  );
}, (prev, next) => prev.item.id === next.item.id);
