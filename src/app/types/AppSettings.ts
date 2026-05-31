import z from "zod";
import { ModelProfileSchema } from "./ModelProfile";

export const AppSettingsSchema = z.object({
  theme: z.enum(['dark', 'light']).default('dark'), // 'gtk-theme'
  voice: z.string().default('en-US-AvaMultilingualNeural'),

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
});

export type AppSettings = z.infer<typeof AppSettingsSchema>;
