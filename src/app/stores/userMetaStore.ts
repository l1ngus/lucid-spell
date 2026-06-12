import { LazyStore } from '@tauri-apps/plugin-store';
import type { LangPair } from '../types/Langs';
import { DEFAULT_USER_META } from '../consts/defaultStoresValues';

export interface UserMeta {
  lastLangPair: LangPair;
}

const store = new LazyStore('user-meta.json');

export async function setMetaItem<K extends keyof UserMeta>(key: K, value: UserMeta[K]) {
  await store.set(key, value);
  await store.save();
}

export async function getMetaItem<K extends keyof UserMeta>(key: K): Promise<UserMeta[K] | undefined> {
  const value = await store.get<UserMeta[K]>(key);
  return value ?? DEFAULT_USER_META[key];
}

export async function getAllMetaItems() {
  const entries = await store.entries<any>();
  const allSettings = Object.fromEntries(entries);
  return { ...DEFAULT_USER_META, ...allSettings } as UserMeta;
}
