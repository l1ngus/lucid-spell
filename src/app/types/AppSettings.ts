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

import z from "zod";
import { type TranslationEngine } from "@/bindings";
import { ModelProfileSchema } from "./ModelProfile";

export const AppSettingsSchema = z.object({
  theme: z.enum(['dark', 'light']).default('dark'), // 'gtk-theme'
  voice: z.string().default('en-US-AvaMultilingualNeural'),

  translationEngine: z.custom<TranslationEngine>().default('google'),

  activeLlmProfileId: z.string().default(''),
  llmProfiles: z.array(ModelProfileSchema).default([]),

  isAutoAltTransFetchEnabled: z.boolean().default(false),
  isAutoLanguageSwitchEnabled: z.boolean().default(true),
  isAutoTranslateEnabled: z.boolean().default(false),
  autoTranslateDelay: z.number().min(0).max(5000).default(700),

  isProxyEnabled: z.boolean().default(false),
  proxyProtocol: z.enum(['socks5', 'https', 'http']).default('socks5'),
  proxyHost: z.string().default(''),
  proxyPort: z.string().default(''),
  proxyUser: z.string().default(''),
  proxyPass: z.string().default(''),

  clearShortcut: z.array(z.string()).default([]),
  swapLangsShortcut: z.array(z.string()).default([]),
  applyCorrectionShortcut: z.array(z.string()).default([]),
  prevFlashcardShortcut: z.array(z.string()).default([]),
  nextFlashcardShortcut: z.array(z.string()).default([]),
  flipFlashcardShortcut: z.array(z.string()).default([])
});

export type AppSettings = z.infer<typeof AppSettingsSchema>;
