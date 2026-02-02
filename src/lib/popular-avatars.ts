import {
  searchBooth,
  getPopularAvatars as getPopularAvatarsApi,
  checkAvatarsNeedUpdate,
  updatePopularAvatar,
  type PopularAvatar,
} from "./booth-api";

export type { PopularAvatar };

export async function getPopularAvatars(): Promise<PopularAvatar[]> {
  return getPopularAvatarsApi();
}

export async function needsUpdate(): Promise<boolean> {
  return checkAvatarsNeedUpdate();
}

export async function updateAvatarData(): Promise<void> {
  const avatars = await getPopularAvatars();

  for (const avatar of avatars) {
    try {
      const result = await searchBooth({
        keyword: `${avatar.name_ja} 対応`,
        page: 1,
        category: "3D衣装",
      });

      const thumbnailUrl =
        result.items[0]?.images[0] ?? avatar.thumbnail_url;

      await updatePopularAvatar(
        avatar.id,
        result.total_count ?? result.items.length,
        thumbnailUrl,
      );

      // Rate limit: wait between requests
      await new Promise((r) => setTimeout(r, 1500));
    } catch (e) {
      console.error(`Failed to update avatar ${avatar.name_ja}:`, e);
    }
  }
}
