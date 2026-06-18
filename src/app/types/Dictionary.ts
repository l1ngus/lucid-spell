import { LangName } from "./Langs"

export interface TermPair {
  source: string;
  target: string;
}

export interface DictionaryPair {
  id: string
  source: string
  target: string
  sourceLang?: LangName
  targetLang?: LangName
}

export interface DictionaryMeta {
  id: string
  name: string
  description?: string
  sourceLang?: string
  targetLang?: string

  createdAt: number
  updatedAt: number

  isFavorites?: boolean
}

export interface Dictionary {
  meta: DictionaryMeta
  pairs: DictionaryPair[]
}
