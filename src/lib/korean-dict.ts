export interface DictEntry {
  ko: string;
  ja: string;
  category: "avatar" | "item" | "vrc";
}

export const AVATAR_DICT: DictEntry[] = [
  { ko: "시나노", ja: "しなの", category: "avatar" },
  { ko: "마누카", ja: "マヌカ", category: "avatar" },
  { ko: "세레스티아", ja: "セレスティア", category: "avatar" },
  { ko: "릴리엘", ja: "リリエル", category: "avatar" },
  { ko: "큐피드", ja: "キュピッド", category: "avatar" },
  { ko: "카린", ja: "カリン", category: "avatar" },
  { ko: "루슈카", ja: "ルシュカ", category: "avatar" },
  { ko: "마이", ja: "舞", category: "avatar" },
  { ko: "사쿠라", ja: "桜", category: "avatar" },
  { ko: "코코아", ja: "ここあ", category: "avatar" },
  { ko: "이메리스", ja: "イメリス", category: "avatar" },
  { ko: "미란", ja: "ミラン", category: "avatar" },
  { ko: "우루루", ja: "うるる", category: "avatar" },
  { ko: "이치고", ja: "イチゴ", category: "avatar" },
  { ko: "키쿄", ja: "桔梗", category: "avatar" },
];

export const ITEM_DICT: DictEntry[] = [
  { ko: "원피스", ja: "ワンピース", category: "item" },
  { ko: "치마", ja: "スカート", category: "item" },
  { ko: "바지", ja: "パンツ", category: "item" },
  { ko: "셔츠", ja: "シャツ", category: "item" },
  { ko: "신발", ja: "シューズ", category: "item" },
  { ko: "부츠", ja: "ブーツ", category: "item" },
  { ko: "모자", ja: "帽子", category: "item" },
  { ko: "안경", ja: "メガネ", category: "item" },
  { ko: "귀걸이", ja: "イヤリング", category: "item" },
  { ko: "목걸이", ja: "ネックレス", category: "item" },
  { ko: "양말", ja: "ソックス", category: "item" },
  { ko: "장갑", ja: "グローブ", category: "item" },
  { ko: "가방", ja: "バッグ", category: "item" },
  { ko: "날개", ja: "翼", category: "item" },
  { ko: "꼬리", ja: "しっぽ", category: "item" },
  { ko: "귀", ja: "耳", category: "item" },
  { ko: "머리카락", ja: "ヘアー", category: "item" },
  { ko: "헤어", ja: "ヘアー", category: "item" },
  { ko: "속옷", ja: "下着", category: "item" },
  { ko: "수영복", ja: "水着", category: "item" },
  { ko: "제복", ja: "制服", category: "item" },
  { ko: "메이드복", ja: "メイド服", category: "item" },
];

export const VRC_DICT: DictEntry[] = [
  { ko: "대응", ja: "対応", category: "vrc" },
  { ko: "전용", ja: "専用", category: "vrc" },
  { ko: "의상", ja: "衣装", category: "vrc" },
  { ko: "소품", ja: "小道具", category: "vrc" },
  { ko: "텍스처", ja: "テクスチャ", category: "vrc" },
  { ko: "아바타", ja: "アバター", category: "vrc" },
  { ko: "월드", ja: "ワールド", category: "vrc" },
  { ko: "기믹", ja: "ギミック", category: "vrc" },
  { ko: "셰이더", ja: "シェーダー", category: "vrc" },
  { ko: "파티클", ja: "パーティクル", category: "vrc" },
  { ko: "무료", ja: "無料", category: "vrc" },
];

export const ALL_DICT: DictEntry[] = [...AVATAR_DICT, ...ITEM_DICT, ...VRC_DICT];
