import { Textarea } from '@/components/ui/textarea';
import { Spinner } from '@/components/ui/spinner';
import { Volume2 } from 'lucide-react';
import useTranslation from '../hooks/useTranslation';
import { cn } from '@/lib/utils';
import useTextToSpeech from '../hooks/useTextToSpeech';


export default function () {
  const {
    translationResult: { response, isFetching, isError, error },
  } = useTranslation();
  const { speak } = useTextToSpeech();

  const handleSpeak = () => {
    speak(response.translation);
  }

  return (
    <div className="relative">
      <div aria-busy={isFetching} className={cn(
        'transition-opacity duration-400 ease-in-out',
        isFetching && 'opacity-50'
      )}>
        <Textarea className={cn(
          'flex-1 resize-none min-h-40',
          isError && 'text-red-400'
        )} readOnly value={isError ? error?.message : response.translation} />
        <div className="absolute bottom-1.5 right-1.5 flex items-center gap-2 p-0.5">
          <Volume2 onClick={handleSpeak} className='cursor-pointer' />
        </div>
      </div>
      {isFetching && <Spinner className='size-8 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2' />}
    </div>
  )
}
