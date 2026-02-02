import FavoritesList from "../components/favorites/FavoritesList";
import { UI_TEXT } from "../lib/constants";

export default function FavoritesPage() {
  return (
    <div className="p-6">
      <div className="max-w-7xl mx-auto">
        <h2 className="text-xl font-bold text-gray-900 mb-6">
          {UI_TEXT.favorites.title}
        </h2>
        <FavoritesList />
      </div>
    </div>
  );
}
