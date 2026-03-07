import { useQuery, type UseQueryResult } from '@tanstack/react-query';
import useTranslator, { type TranslateParams } from './useTranslator';
import { type TranslateResponse } from '../types/TranslateResponse';


export type UseTranslateQueryOptions = TranslateParams;
export type UseTranslateQueryResult = {
  response: TranslateResponse;
} & Pick<UseQueryResult<TranslateResponse>, 'isFetching' | 'isError' | 'error'>

export default ({ term, sourceLang, targetLang }: UseTranslateQueryOptions): UseTranslateQueryResult => {
  const { translateViaLlm } = useTranslator();

  const isEnabled = !!term && term.trim().length > 0;

  const { data, isFetching, isError, error } = useQuery({
    queryKey: ['transalte-key', term, sourceLang, targetLang],
    queryFn: () => translateViaLlm({
      term,
      sourceLang,
      targetLang
    }),
    enabled: isEnabled,
    retry: false,
  })

  const response = data ?? { translation: '', sourceCorrection: '' };

  return {
    response,
    isFetching,
    isError,
    error
  }
}
