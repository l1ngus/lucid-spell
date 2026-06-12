import z from "zod";
import { languagesByName } from "../consts/languages";

const langCodeValues = Object.values(languagesByName) as [string, ...string[]];

export const LangCodeSchema = z.enum(langCodeValues);

export const LangPairSchema = z.object({
  source: z.union([LangCodeSchema, z.literal('auto')]).default('auto'),
  target: LangCodeSchema.default('eng'),
});

export const UserMetaSchema = z.object({
  lastLangPair: LangPairSchema.default({ source: 'auto', target: 'eng' }),
});

export type UserMeta = z.infer<typeof UserMetaSchema>;
