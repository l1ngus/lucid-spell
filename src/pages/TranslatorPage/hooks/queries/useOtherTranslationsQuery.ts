import { useQuery, type UseQueryResult } from '@tanstack/react-query';
import { commands } from '@/bindings';
import useActiveLlmProfile from '../useActiveLlmProfile';
import { LangCode } from '@/app/types/Langs';
import { TranslateOthersResponseScheme, TranslateOthersWithPartsResponseScheme } from '../../types/TranslateResponse';

export interface UseOtherTranslationsQueryOptions {
  sourceText: string;
  translatedText: string;
  sourceLang: LangCode | 'auto';
  targetLang: LangCode;
  withParts: boolean; // with parts of speech
}
// export type UseOtherTranslationsQueryResult = 

export default () => {
  const llmProfile = useActiveLlmProfile();

  const fetchOtherTranslations = async ({ sourceText, translatedText, sourceLang, targetLang, withParts }: UseOtherTranslationsQueryOptions) => {
    const prompt = withParts ? "withparts" : "without";
    const response = await commands.askLlm([{
      role: 'user',
      content: prompt
    }], llmProfile.model, .7);
    if (response.status === 'error')
      throw new Error(response.error);
    const cleanStr = response.data
      .replace(/^(```|""")\w*\n/, "")
      .replace(/(```|""")$/, "");
    const parsedStr = JSON.parse(cleanStr);

    const result = TranslateOthersWithPartsResponseScheme.parse(parsedStr); /// It should check another response scheme!!!!!!
    return result;
  }
}
