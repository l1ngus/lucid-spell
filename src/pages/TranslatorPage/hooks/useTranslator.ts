import { invoke } from '@tauri-apps/api/core';
import { commands } from '@/bindings';
import { type LangDetectionResult } from '@/app/types/LangDetectionResult';
import type { LangCode } from "@/app/types/Langs";
import { getTranslationPrompt } from "@/app/consts/prompts";
import { type TranslateResponse, TranslateResponseScheme } from '../types/TranslateResponse'
import useActiveLlmProfile from './useActiveLlmProfile';


export interface TranslateParams {
  term: string;
  sourceLang: LangCode | 'auto';
  targetLang: LangCode;
}

export default () => {
  const llmProfile = useActiveLlmProfile();

  const translateViaLlm = async ({ term, sourceLang, targetLang }: TranslateParams): Promise<TranslateResponse> => {
    const prompt = getTranslationPrompt({ text: term, sourceLang, targetLang });
    const response = await commands.askLlm([{
      role: 'user',
      content: prompt
    }], llmProfile.model, .7);

    if (response.status === 'error')
      throw new Error(response.error);

    const cleanStr = response.data
      .replace(/^(```|""")\w*\n/, "")
      .replace(/(```|""")$/, "");

    const result = TranslateResponseScheme.parse(JSON.parse(cleanStr));
    return result;
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
