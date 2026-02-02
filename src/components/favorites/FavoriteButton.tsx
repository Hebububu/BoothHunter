import { Heart } from "lucide-react";
import { clsx } from "clsx";
import { useFavorites } from "../../hooks/useFavorites";
import type { BoothItem } from "../../lib/types";

interface Props {
  item: BoothItem;
}

export default function FavoriteButton({ item }: Props) {
  const { isFavorite, addFavorite, removeFavorite } = useFavorites();
  const favorited = isFavorite(item.id);

  const handleClick = async (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    try {
      if (favorited) {
        await removeFavorite(item.id);
      } else {
        await addFavorite(item);
      }
    } catch (err) {
      console.error("Favorite operation failed:", err);
    }
  };

  return (
    <button
      onClick={handleClick}
      className={clsx(
        "p-1 rounded-md transition-colors shrink-0",
        favorited
          ? "text-red-500 hover:text-red-600"
          : "text-gray-300 hover:text-red-400",
      )}
      title={favorited ? "즐겨찾기 제거" : "즐겨찾기 추가"}
    >
      <Heart
        className="w-4 h-4"
        fill={favorited ? "currentColor" : "none"}
      />
    </button>
  );
}
