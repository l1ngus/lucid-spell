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

import { invoke } from '@tauri-apps/api/core';
import { commands, TranslationRequest, TranslationResponse } from '@/bindings';
import { type LangDetectionResult } from '@/app/types/LangDetectionResult';
import type { LangCode } from "@/app/types/Langs";
// import { getTranslationPrompt } from "@/app/consts/prompts";
// import { type TranslateResponse, TranslateResponseScheme } from '../types/TranslateResponse'
import useActiveLlmProfile from './useActiveLlmProfile';
// import { extractAndParseJSON } from '@/app/helpers/parseLlmRespone';


export default () => {
  const llmProfile = useActiveLlmProfile();

  const translateViaLlm = async (transRequest: TranslationRequest): Promise<TranslationResponse> => {
    if (!llmProfile) throw new Error("No LLM profile selected. Go to Settings to add one.");

    const response = await commands.translate(transRequest);

    if (response.status === 'error') {
      throw new Error(response.error);
    }

    // const parsedData = extractAndParseJSON(response.data);
    // const result = TranslateResponseScheme.parse(parsedData);

    return response.data;
  }

  const detectLang = async (text: string, whitelist?: LangCode[]) => {
    const res = await invoke("detect_language", { text, whitelist }) as LangDetectionResult;
    return res;
  }

  return {
    translateViaLlm,
    detectLang
  }
}
