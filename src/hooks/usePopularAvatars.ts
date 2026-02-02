import { useEffect, useRef } from "react";
import { useQuery } from "@tanstack/react-query";
import {
  getPopularAvatars,
  needsUpdate,
  updateAvatarData,
  type PopularAvatar,
} from "../lib/popular-avatars";

// Module-level lock to prevent concurrent background updates across hook instances
let updateInProgress = false;

export function usePopularAvatars() {
  const query = useQuery<PopularAvatar[]>({
    queryKey: ["popular-avatars"],
    queryFn: getPopularAvatars,
    staleTime: 60 * 60 * 1000, // 1 hour
  });

  const refetchRef = useRef(query.refetch);
  refetchRef.current = query.refetch;

  // Background update if data is stale (>7 days)
  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        if (updateInProgress) return;
        if (await needsUpdate()) {
          if (cancelled || updateInProgress) return;
          updateInProgress = true;
          try {
            await updateAvatarData();
            if (!cancelled) {
              refetchRef.current();
            }
          } finally {
            updateInProgress = false;
          }
        }
      } catch (e) {
        console.error("Background avatar update failed:", e);
        updateInProgress = false;
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  return {
    avatars: query.data ?? [],
    isLoading: query.isLoading,
  };
}
