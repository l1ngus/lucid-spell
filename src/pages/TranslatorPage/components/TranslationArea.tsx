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

import { useEffect, useState } from 'react';
import { Textarea } from '@/components/ui/textarea';
import { Spinner } from '@/components/ui/spinner';
import { Volume2, Heart, BookPlus } from 'lucide-react';
import useTranslation from '../hooks/useTranslation';
import { cn } from '@/lib/utils';
import useTextToSpeech from '../hooks/useTextToSpeech';
import { addFavoriteTranslation, isTranslationFavorite, removeFavoriteTranslationByContent } from '@/app/stores/favoritesStore';
import ChooseDictDialog from './ChooseDictDialog';
import { addPair } from '@/app/stores/dictionariesStore';


export default function () {
  const {
    translationResult: { response, isFetching, isError, error },
    sourceText,
    langPair,
    ocrError
  } = useTranslation();
  const { speak } = useTextToSpeech();
  const [isLiked, setIsLiked] = useState(false);

  useEffect(() => {
    if (!response.translation || !sourceText) {
      setIsLiked(false);
      return;
    }
    isTranslationFavorite(sourceText.trim(), response.translation.trim(), langPair)
      .then(isFav => setIsLiked(isFav));
  }, [response.translation, sourceText, langPair])

  const handleSpeak = () => {
    speak(response.translation);
  }

  const handleLike = async () => {
    if (!sourceText || !response.translation) return;
    try {
      if (isLiked)
        await removeFavoriteTranslationByContent(sourceText.trim(), response.translation.trim(), langPair);
      else
        await addFavoriteTranslation(sourceText.trim(), response.translation.trim(), langPair);
      setIsLiked(prev => !prev);
    } catch (e) {
      // handle error silently or show toast
      console.error(e);
    }
  };

  const handleDictSelect = (dictId: string) => {
    const pair = {
      source: sourceText,
      target: response.translation,
    }
    addPair(dictId, pair);
  }

  const displayedTranslation = ocrError ?? (isError ? error?.message : response.translation);

  return (
    <div className="relative">
      <div aria-busy={isFetching} className={cn(
        'transition-opacity duration-400 ease-in-out',
        isFetching && 'opacity-50'
      )}>
        <Textarea className={cn(
          'flex-1 resize-none min-h-40',
          (isError || ocrError) && 'text-destructive'
        )} readOnly aria-invalid={isError || Boolean(ocrError)} aria-live={ocrError ? 'assertive' : undefined} value={displayedTranslation} />
        <div className="absolute bottom-1.5 right-1.5 flex items-center gap-2 p-0.5">
          <Heart onClick={handleLike} className={cn('cursor-pointer', isLiked && 'fill-destructive text-destructive')} />
          <ChooseDictDialog onDictSelect={handleDictSelect} checkWhetherOpen={() => Boolean(sourceText) || Boolean(response.translation)}>
            <BookPlus className='cursor-pointer' />
          </ChooseDictDialog>
          <Volume2 onClick={handleSpeak} className='cursor-pointer' />
        </div>
      </div>
      {isFetching && <Spinner className='size-8 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2' />}
    </div >
  )
}
