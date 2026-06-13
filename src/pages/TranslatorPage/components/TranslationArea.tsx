import { useEffect, useState } from 'react';
import { Textarea } from '@/components/ui/textarea';
import { Spinner } from '@/components/ui/spinner';
import { Volume2, Heart, BookPlus } from 'lucide-react';
import useTranslation from '../hooks/useTranslation';
import { cn } from '@/lib/utils';
import useTextToSpeech from '../hooks/useTextToSpeech';
import { addFavoriteTranslation, isTranslationFavorite, removeFavoriteTranslationByContent } from '@/app/stores/favoritesStore';
import ChooseDictDiolog from './ChooseDictDiolog';


export default function () {
  const {
    translationResult: { response, isFetching, isError, error },
    sourceText,
    langPair
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

  const handleAddToDict = () => {

  }

  return (
    <div className="relative">
      <div aria-busy={isFetching} className={cn(
        'transition-opacity duration-400 ease-in-out',
        isFetching && 'opacity-50'
      )}>
        <Textarea className={cn(
          'flex-1 resize-none min-h-40',
          isError && 'text-destructive'
        )} readOnly value={isError ? error?.message : response.translation} />
        <div className="absolute bottom-1.5 right-1.5 flex items-center gap-2 p-0.5">
          <Heart onClick={handleLike} className={cn('cursor-pointer', isLiked && 'fill-destructive text-destructive')} />
          <ChooseDictDiolog>
            <BookPlus onClick={handleAddToDict} className='cursor-pointer' />
          </ChooseDictDiolog>
          <Volume2 onClick={handleSpeak} className='cursor-pointer' />
        </div>
      </div>
      {isFetching && <Spinner className='size-8 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2' />}
    </div>
  )
}
