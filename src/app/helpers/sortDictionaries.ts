import { type Dictionary } from "../types/Dictionary";

export default (dicts: Dictionary[]) => {
  const sorted = [...dicts].sort((a, b) =>
    b.meta.updatedAt - a.meta.updatedAt
  );
  const favIndex = sorted.findIndex(d => d.meta.isFavorites);
  if (favIndex !== -1) {
    const [fav] = sorted.splice(favIndex, 1);
    sorted.unshift(fav);
  }
  return sorted;
};
