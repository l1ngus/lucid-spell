import { z } from 'zod';

// simple translate response one source, one target
export const TranslateResponseScheme = z.object({
  translation: z.string(),
  sourceCorrection: z.string().default('')
});
export type TranslateResponse = z.infer<typeof TranslateResponseScheme>;

// response with only other translations
export const TranslateOthersResponseScheme = z.object({
  otherTranslations: z.array(z.string())
});
export type TranslateOthersResponse = z.infer<typeof TranslateOthersResponseScheme>;


// response with other translations with parts of speech
export const TranslateOthersWithPartsResponseScheme = z.object({
  otherTranslations: z.array(z.object({
    parts: z.string(),
    translation: z.string()
  }))
});
export type TranslateOthersWithPartsResponse = z.infer<typeof TranslateOthersWithPartsResponseScheme>;


