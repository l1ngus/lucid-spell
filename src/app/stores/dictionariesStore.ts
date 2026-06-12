import { LazyStore } from '@tauri-apps/plugin-store';
import type { Dictionary, DictionaryMeta, DictionaryPair } from '../types/Dictionary';
import { DEFAULT_DICTIONARIES } from '../consts/defaultStoresValues';

const STORE_KEY = 'dictionaries';
const store = new LazyStore('dictionaries.json');

function generateId(): string {
  return crypto.randomUUID();
}

export async function getAllDictionaries(): Promise<Dictionary[]> {
  const entries = await store.entries<Dictionary[]>();
  const map = Object.fromEntries(entries);
  return map[STORE_KEY] ?? DEFAULT_DICTIONARIES;
}

export async function getDictionary(id: string): Promise<Dictionary | undefined> {
  const dicts = await getAllDictionaries();
  return dicts.find(d => d.meta.id === id);
}

export async function createDictionary(
  params: Pick<DictionaryMeta, 'name' | 'description' | 'sourceLang' | 'targetLang'>
): Promise<Dictionary> {
  const dicts = await getAllDictionaries();
  const now = Date.now();
  const newDict: Dictionary = {
    meta: {
      id: generateId(),
      name: params.name,
      description: params.description,
      sourceLang: params.sourceLang,
      targetLang: params.targetLang,
      createdAt: now,
      updatedAt: now,
    },
    pairs: [],
  };
  dicts.push(newDict);
  await store.set(STORE_KEY, dicts);
  await store.save();
  return newDict;
}

export async function updateDictionaryMeta(
  id: string,
  updates: Partial<Pick<DictionaryMeta, 'name' | 'description' | 'sourceLang' | 'targetLang'>>
): Promise<void> {
  const dicts = await getAllDictionaries();
  const dict = dicts.find(d => d.meta.id === id);
  if (!dict) return;
  Object.assign(dict.meta, updates, { updatedAt: Date.now() });
  await store.set(STORE_KEY, dicts);
  await store.save();
}

export async function deleteDictionary(id: string): Promise<void> {
  const dicts = await getAllDictionaries();
  const filtered = dicts.filter(d => d.meta.id !== id);
  await store.set(STORE_KEY, filtered);
  await store.save();
}

export async function addPair(
  dictionaryId: string,
  pair: Pick<DictionaryPair, 'source' | 'target' | 'sourceLang' | 'targetLang'>
): Promise<DictionaryPair | null> {
  const dicts = await getAllDictionaries();
  const dict = dicts.find(d => d.meta.id === dictionaryId);
  if (!dict) return null;
  const newPair: DictionaryPair = { id: generateId(), ...pair };
  dict.pairs.push(newPair);
  dict.meta.updatedAt = Date.now();
  await store.set(STORE_KEY, dicts);
  await store.save();
  return newPair;
}

export async function updatePair(
  dictionaryId: string,
  pairId: string,
  updates: Partial<Pick<DictionaryPair, 'source' | 'target'>>
): Promise<void> {
  const dicts = await getAllDictionaries();
  const dict = dicts.find(d => d.meta.id === dictionaryId);
  if (!dict) return;
  const pair = dict.pairs.find(p => p.id === pairId);
  if (!pair) return;
  Object.assign(pair, updates);
  dict.meta.updatedAt = Date.now();
  await store.set(STORE_KEY, dicts);
  await store.save();
}

export async function removePair(dictionaryId: string, pairId: string): Promise<void> {
  const dicts = await getAllDictionaries();
  const dict = dicts.find(d => d.meta.id === dictionaryId);
  if (!dict) return;
  dict.pairs = dict.pairs.filter(p => p.id !== pairId);
  dict.meta.updatedAt = Date.now();
  await store.set(STORE_KEY, dicts);
  await store.save();
}
