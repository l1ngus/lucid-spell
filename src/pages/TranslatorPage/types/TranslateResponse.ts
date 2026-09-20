/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { z } from 'zod';

// // simple translate response one source, one target
// export const TranslateResponseScheme = z.object({
//   translation: z.string(),
//   sourceCorrection: z.string().default('')
// });
// export type TranslateResponse = z.infer<typeof TranslateResponseScheme>;

// response with only other translations
export const TranslateOthersResponseScheme = z.object({
  otherTranslations: z.array(z.string())
});
export type TranslateOthersResponse = z.infer<typeof TranslateOthersResponseScheme>;


// response with other translations with parts of speech
export const TranslateOthersWithPartsResponseScheme = z.object({
  otherTranslations: z.array(z.object({
    part: z.string(),
    translation: z.string()
  }))
});
export type TranslateOthersWithPartsResponse = z.infer<typeof TranslateOthersWithPartsResponseScheme>;


