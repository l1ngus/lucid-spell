export interface DictionaryPair {
  id: string
  source: string
  target: string
}

export interface DictionaryMeta {
  id: string
  name: string
  description?: string
  sourceLang?: string
  targetLang?: string
  createdAt: number
  updatedAt: number
}

export interface Dictionary {
  meta: DictionaryMeta
  pairs: DictionaryPair[]
}
