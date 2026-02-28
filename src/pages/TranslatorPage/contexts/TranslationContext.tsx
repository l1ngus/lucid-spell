import { useMemo, useCallback, useState, createContext, type PropsWithChildren } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import type { LangPair } from '@/app/types/Langs';
import { DETECT_AND_SWAP_QUERY_KEY } from '../hooks/useDetectAndSwap';
import useTranslateQuery, { type UseTranslateQueryResult } from '../hooks/useTranslateQuery';
import { useDebouncedCallback } from '@/app/hooks/useDebouncedCallback';
import useSettings from '@/app/hooks/useSettings';
import useUserMeta from '@/app/hooks/useUserMeta';

type UpdateLangPairFn = (value: Partial<LangPair>) => void;
type UpdateSourceTextFn = (value: string) => void;

export interface TranslationContextValue {
  translationResult: UseTranslateQueryResult;
  langPair: LangPair;
  translateCurrent: () => void;
  updateLangPair: UpdateLangPairFn;
  sourceText: string;
  updateSourceText: UpdateSourceTextFn;
  swapLangs: (swapTranslation?: boolean) => void;
}

export const TranslationContext = createContext<TranslationContextValue | null>(null);

export const TranslationProvider = ({ children }: PropsWithChildren) => {
  const { settings } = useSettings();
  const { getUserMeta, setUserMeta } = useUserMeta();
  const queryClient = useQueryClient();
  const [langPair, setLangPair] = useState<LangPair>(getUserMeta('lastLangPair'));
  const [sourceText, setSourceText] = useState('');
  const [textForQuery, setTextForQuery] = useState('');
  const translationResult = useTranslateQuery({ term: textForQuery, sourceLang: langPair.source, targetLang: langPair.target });

  const [setTextForQueryDebounced, preventChangingTextForQuery]
    = useDebouncedCallback((value: string) => setTextForQuery(value), settings.autoTranslateDelay);

  const updateLangPair: UpdateLangPairFn = useCallback((value) => {
    queryClient.cancelQueries({ queryKey: DETECT_AND_SWAP_QUERY_KEY });
    setLangPair(prev => {
      const newValue = { ...prev, ...value };
      setUserMeta('lastLangPair', newValue);
      return newValue;
    });
  }, [])

  const updateSourceText: UpdateSourceTextFn = useCallback((value) => {
    setSourceText(value);
    const trimmedValue = value.trim();
    if (settings.isAutoTranslateEnabled)
      if (trimmedValue)
        setTextForQueryDebounced(trimmedValue);
      else {
        preventChangingTextForQuery();
        setTextForQuery('');
      }
  }, [settings.isAutoTranslateEnabled]);

  const swapLangs = useCallback((swapTranslation: boolean = false) => {
    setLangPair(prev => {
      if (prev.source === 'auto') return prev;
      if (swapTranslation && translationResult.translation) {
        setSourceText(translationResult.translation);
        setTextForQuery(translationResult.translation);
      }
      return { source: prev.target, target: prev.source };
    });
  }, [translationResult.translation]);

  const translateCurrent = useCallback(() => setTextForQuery(sourceText), [sourceText]);

  const contextValue = useMemo<TranslationContextValue>(() => ({
    translationResult,
    langPair,
    translateCurrent,
    updateLangPair,
    sourceText,
    updateSourceText,
    swapLangs
  }), [translationResult, langPair, translateCurrent, updateLangPair, sourceText, swapLangs])

  return (
    <TranslationContext.Provider value={contextValue}>
      {children}
    </TranslationContext.Provider>
  )
}
