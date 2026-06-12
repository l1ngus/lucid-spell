import { LazyStore } from '@tauri-apps/plugin-store';
import { UserMetaSchema, type UserMeta } from '../types/UserMeta';
export type { UserMeta };

const DEFAULT_USER_META = UserMetaSchema.parse({});

const store = new LazyStore('user-meta.json');

export async function setMetaItem<K extends keyof UserMeta>(key: K, value: UserMeta[K]) {
  await store.set(key, value);
  await store.save();
}

export async function getMetaItem<K extends keyof UserMeta>(key: K): Promise<UserMeta[K] | undefined> {
  const value = await store.get<UserMeta[K]>(key);
  return value ?? DEFAULT_USER_META[key];
}

export async function getAllMetaItems(): Promise<UserMeta> {
  const entries = await store.entries<any>();
  const allSettings = Object.fromEntries(entries);
  return UserMetaSchema.catch(UserMetaSchema.parse({})).parse(allSettings);
}
