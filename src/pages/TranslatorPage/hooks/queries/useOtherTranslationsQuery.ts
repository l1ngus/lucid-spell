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

import { useQuery, type UseQueryResult } from '@tanstack/react-query';
import { commands, type OtherTranslationsResponse, type TranslationEngine } from '@/bindings';
import { LangCode } from '@/app/types/Langs';
import { supportsOtherTranslations } from '../../lib/translationCapabilities';

export interface UseOtherTranslationsQueryOptions {
  engine: TranslationEngine;
  sourceText: string;
  translatedText: string;
  sourceLang: LangCode | 'auto';
  targetLang: LangCode;
  maxSourceLength?: number;
  isEnabled?: boolean;
}

export type UseOtherTranslationsQueryResult = {
  response: OtherTranslationsResponse;
} & Pick<UseQueryResult<OtherTranslationsResponse>, 'isFetching' | 'isError' | 'error'>

export default (translateOptions: UseOtherTranslationsQueryOptions): UseOtherTranslationsQueryResult => {
  const { engine, sourceText, translatedText, sourceLang, targetLang } = translateOptions;

  const fetchOtherTranslations = async (): Promise<OtherTranslationsResponse> => {
    const response = await commands.getOtherTranslations({
      engine,
      sourceText,
      translatedText,
      sourceLang,
      targetLang
    });
    if (response.status === 'error')
      throw new Error(response.error);
    return response.data;
  }

  const isEnabled = supportsOtherTranslations(engine)
    && (translateOptions.isEnabled || typeof (translateOptions.isEnabled) === 'undefined')
    && (
      !!translatedText
      && (!translateOptions.maxSourceLength || sourceText.length <= translateOptions.maxSourceLength)
    );

  const { data, isFetching, isError, error } = useQuery({
    queryKey: ['translate-other-key',
      engine,
      sourceText,
      translatedText,
      sourceLang,
      targetLang
    ],
    queryFn: fetchOtherTranslations,
    enabled: isEnabled,
    retry: false,
    staleTime: Infinity,
    gcTime: 1000 * 60 * 60 * 24,
  })

  return { response: data ?? { otherTranslations: [] }, isFetching, isError, error };
}
