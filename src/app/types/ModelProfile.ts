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
import { AiServiceSchema } from "./AiService";

export const ModelProfileSchema = z.object({
  id: z.string(),
  name: z.string(),
  aiService: AiServiceSchema,
  serviceUrl: z.string(),
  // apiKey: z.string(),
  // isProxyEnabled: z.boolean().default(false),
  model: z.string(),
  temperature: z.number().min(0.0).max(2.0).default(0.5)
});


export type ModelProfile = z.infer<typeof ModelProfileSchema>;
