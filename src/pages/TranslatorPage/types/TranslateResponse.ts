import { z } from 'zod';

export const TranslateResponseScheme = z.object({
  translation: z.string(),
  sourceCorrection: z.string().default('')
});

export type TranslateResponse = z.infer<typeof TranslateResponseScheme>;
