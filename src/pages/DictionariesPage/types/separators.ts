import { INNER_SEPARATORS, OUTER_SEPARATORS } from "../consts/Separators";

export type InnerSeparator = keyof typeof INNER_SEPARATORS;
export type OuterSeparator = keyof typeof OUTER_SEPARATORS;

export interface SepPair {
  inner: InnerSeparator;
  outer: OuterSeparator;
}
