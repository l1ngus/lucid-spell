import { UserMeta } from '../stores/userMetaStore';
import type { Dictionary } from '../types/Dictionary';

export const DEFAULT_USER_META: UserMeta = {
  lastLangPair: { source: 'auto', target: 'eng' }
} as const;

export const DEFAULT_DICTIONARIES: Dictionary[] = [];
