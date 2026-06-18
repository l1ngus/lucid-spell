import { INNER_SEPARATORS, OUTER_SEPARATORS } from "../consts/Separators"
import { type SepPair } from "../types/separators"

const getPairsPreview = (sepPair: SepPair) => {
  const in_sep = INNER_SEPARATORS[sepPair.inner].sep;
  const out_sep = OUTER_SEPARATORS[sepPair.outer].sep;

  const preview = `term1${in_sep}translation1${out_sep}term2${in_sep}translation2`;
  return preview
}

export default getPairsPreview;
