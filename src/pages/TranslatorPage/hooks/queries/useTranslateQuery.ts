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
import useTranslator from '../useTranslator';
import { TranslationRequest, type TranslationResponse } from '@/bindings';
// import { type TranslateResponse } from '../../types/TranslateResponse';
import useSettings from '@/app/hooks/useSettings';


export type UseTranslateQueryOptions = TranslationRequest;
export type UseTranslateQueryResult = {
  response: TranslationResponse
} & Pick<UseQueryResult<TranslationResponse>, 'isFetching' | 'isError' | 'error'>

export default ({ engine, text, sourceLang, targetLang }: UseTranslateQueryOptions): UseTranslateQueryResult => {
  const { settings } = useSettings();
  const { translateViaLlm } = useTranslator();

  const isEnabled = !!text && text.trim().length > 0;

  const currentProfile = settings.llmProfiles.find(prof => prof.id === settings.activeLlmProfileId);

  const { data, isFetching, isError, error } = useQuery({
    queryKey: [
      'translate',
      {
        text,
        sourceLang,
        targetLang,
        profile: currentProfile
          ? {
            id: currentProfile.id,
            aiService: currentProfile.aiService,
            model: currentProfile.model,
            temperature: currentProfile.temperature,
          }
          : null,
      },
    ],
    queryFn: () => translateViaLlm({
      engine,
      text,
      sourceLang,
      targetLang
    }),
    enabled: isEnabled,
    retry: false,
    placeholderData: undefined,
    staleTime: Infinity,
    gcTime: 1000 * 60 * 60 * 24
  })

  const response = data ?? { translation: '', detectedSourceLang: null, sourceCorrection: null };

  return {
    response,
    isFetching,
    isError,
    error
  }
}
