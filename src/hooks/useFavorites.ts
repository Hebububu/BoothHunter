import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  getFavorites,
  addFavorite as addFavoriteApi,
  removeFavorite as removeFavoriteApi,
} from "../lib/booth-api";
import type { BoothItem, FavoriteItem } from "../lib/types";

export function useFavorites() {
  const queryClient = useQueryClient();

  const favoritesQuery = useQuery({
    queryKey: ["favorites"],
    queryFn: (): Promise<FavoriteItem[]> => getFavorites(),
  });

  const addMutation = useMutation({
    mutationFn: async (item: BoothItem) => {
      await addFavoriteApi({
        item_id: item.id,
        name: item.name,
        price: item.price,
        thumbnail_url: item.images[0] || null,
        category_name: item.category_name,
        shop_name: item.shop_name,
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["favorites"] });
    },
  });

  const removeMutation = useMutation({
    mutationFn: async (itemId: number) => {
      await removeFavoriteApi(itemId);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["favorites"] });
    },
  });

  const isFavorite = (itemId: number): boolean => {
    return (
      favoritesQuery.data?.some((f) => f.item_id === itemId) ?? false
    );
  };

  return {
    favorites: favoritesQuery.data ?? [],
    isLoading: favoritesQuery.isLoading,
    addFavorite: addMutation.mutateAsync,
    removeFavorite: removeMutation.mutateAsync,
    isFavorite,
  };
}
