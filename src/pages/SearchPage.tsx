import { useEffect, useRef } from "react";
import { useSearch } from "../hooks/useSearch";
import { useSearchContext } from "../lib/SearchContext";
import SearchBar from "../components/search/SearchBar";
import SearchResults from "../components/search/SearchResults";
import FilterPanel from "../components/search/FilterPanel";
import AvatarQuickFilter from "../components/search/AvatarQuickFilter";
import Pagination from "../components/common/Pagination";
import { ITEMS_PER_PAGE, UI_TEXT, CATEGORIES } from "../lib/constants";
import { Search, X } from "lucide-react";

export default function SearchPage() {
  const {
    search,
    setPage,
    updateFilters,
    items,
    totalCount,
    currentPage,
    isLoading,
    error,
    hasSearched,
    currentParams,
  } = useSearch();

  const { activeCategory, setActiveCategory } = useSearchContext();
  const prevCategory = useRef(activeCategory);

  // Sync sidebar category → search filters
  useEffect(() => {
    if (activeCategory !== prevCategory.current) {
      prevCategory.current = activeCategory;
      updateFilters({ category: activeCategory ?? undefined });
    }
  }, [activeCategory, updateFilters]);

  const handleSearch = (keyword: string) => {
    search(keyword, activeCategory ? { category: activeCategory } : undefined);
  };

  const handleAvatarSearch = (keyword: string) => {
    search(keyword, activeCategory ? { category: activeCategory } : undefined);
  };

  const handleClearCategory = () => {
    setActiveCategory(null);
    if (currentParams) {
      updateFilters({ category: undefined });
    }
  };

  const totalPages = totalCount
    ? Math.ceil(totalCount / ITEMS_PER_PAGE)
    : items.length > 0
      ? currentPage + 1
      : currentPage;

  const categoryLabel = activeCategory
    ? CATEGORIES[activeCategory] ?? activeCategory
    : null;

  return (
    <div className="p-6">
      <div className="max-w-7xl mx-auto">
        <SearchBar
          onSearch={handleSearch}
          initialKeyword={currentParams?.keyword}
          isLoading={isLoading}
        />

        {/* Active category badge */}
        {categoryLabel && (
          <div className="mt-2 flex items-center gap-2">
            <span className="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium bg-indigo-100 text-indigo-700">
              {categoryLabel}
              <button
                onClick={handleClearCategory}
                className="ml-0.5 hover:text-indigo-900"
              >
                <X className="w-3 h-3" />
              </button>
            </span>
          </div>
        )}

        {/* Avatar Quick Filter */}
        <div className="mt-3">
          <AvatarQuickFilter onSearch={handleAvatarSearch} />
        </div>

        {hasSearched && (
          <div className="mt-4">
            <FilterPanel
              params={currentParams!}
              onFilterChange={updateFilters}
            />
          </div>
        )}

        <div className="mt-6">
          {!hasSearched ? (
            <div className="flex flex-col items-center justify-center py-20 text-gray-400">
              <Search className="w-16 h-16 mb-4" />
              <p className="text-lg">{UI_TEXT.search.placeholder}</p>
              <p className="text-sm mt-1">
                VRChat 아바타, 의상, 소품 등을 검색하세요
              </p>
            </div>
          ) : (
            <>
              <SearchResults
                items={items}
                isLoading={isLoading}
                error={error}
                totalCount={totalCount}
              />
              {items.length > 0 && (
                <div className="mt-6">
                  <Pagination
                    currentPage={currentPage}
                    totalPages={totalPages}
                    onPageChange={setPage}
                  />
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </div>
  );
}
