import { useState, useCallback } from "react";
import { useQuery } from "@tanstack/react-query";
import { searchBooth, cacheItems, saveSearchHistory } from "../lib/booth-api";
import type { SearchParams } from "../lib/types";

export function useSearch() {
  const [params, setParams] = useState<SearchParams | null>(null);

  const query = useQuery({
    queryKey: ["search", params],
    queryFn: async () => {
      if (!params) throw new Error("No params");
      const result = await searchBooth(params);
      // Fire-and-forget DB side effects (non-blocking)
      cacheItems(result.items).catch((e) =>
        console.error("Failed to cache items:", e),
      );
      saveSearchHistory(params.keyword).catch((e) =>
        console.error("Failed to save search history:", e),
      );
      return result;
    },
    enabled: !!params,
  });

  const search = useCallback(
    (keyword: string, extra?: Partial<SearchParams>) => {
      setParams({ keyword, page: 1, ...extra });
    },
    [],
  );

  const setPage = useCallback((page: number) => {
    setParams((prev) => (prev ? { ...prev, page } : null));
  }, []);

  const updateFilters = useCallback((filters: Partial<SearchParams>) => {
    setParams((prev) => {
      if (prev) {
        return { ...prev, ...filters, page: 1 };
      }
      // Allow starting a search with filters only (e.g. category browse)
      return { keyword: "", page: 1, ...filters };
    });
  }, []);

  return {
    search,
    setPage,
    updateFilters,
    items: query.data?.items ?? [],
    totalCount: query.data?.total_count ?? null,
    currentPage: query.data?.current_page ?? 1,
    isLoading: query.isLoading || query.isFetching,
    error: query.error
      ? query.error instanceof Error
        ? query.error.message
        : String(query.error)
      : null,
    hasSearched: !!params,
    currentParams: params,
  };
}
