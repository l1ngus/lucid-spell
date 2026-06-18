
export const INNER_SEPARATORS = {
  dash: {
    label: "Dash (-)",
    sep: "-"
  },
  colon: {
    label: "Colon (:)",
    sep: ":",
  },
  doubleColon: {
    label: "Double colon (::)",
    sep: "::"
  }
}

export const OUTER_SEPARATORS = {
  newLine: {
    label: "New line",
    sep: "\n"
  },
  semicolon: {
    label: "Semicolon (;)",
    sep: ";",
  }
}

import { type InnerSeparator, type OuterSeparator } from "../types/separators";

export const DEFAULT_INNER: InnerSeparator = "doubleColon";
export const DEFAULT_OUTER: OuterSeparator = "newLine";
