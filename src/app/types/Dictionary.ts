export interface DictionaryPair {
  id: string
  source: string
  target: string
}

export interface DictionaryMeta {
  id: string
  name: string
  description: string
  createdAt: number
  updatedAt: number
  sourceLang: string
  targetLang: string
}

export interface Dictionary {
  meta: DictionaryMeta
  pairs: DictionaryPair[]
}
