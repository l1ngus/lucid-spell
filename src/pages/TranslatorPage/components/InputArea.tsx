import { useRef, useEffect, type ChangeEvent } from 'react';
import { Textarea } from '@/components/ui/textarea';
import Tooltip from '@/components/ui/Tooltip';
import { ChevronsRightIcon, MicIcon, Volume2, Lightbulb } from 'lucide-react';
import useTranslation from '../hooks/useTranslation';
import useSettings from '@/app/hooks/useSettings';
import useDetectAndSwap from '../hooks/useDetectAndSwap';
import useTextToSpeech from '../hooks/useTextToSpeech';

export default function () {
  const { settings } = useSettings();
  const detectAndSwapLangs = useDetectAndSwap();
  const { translateCurrent, updateSourceText, langPair, sourceText, translationResult } = useTranslation();
  const { speak } = useTextToSpeech();
  const inputTextareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    const handleFocus = () => {
      inputTextareaRef.current?.focus();
    };
    window.addEventListener('focus', handleFocus);
    return () => {
      window.removeEventListener('focus', handleFocus);
    };
  }, []);

  const handleChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    const sourceValue = e.target.value;
    updateSourceText(sourceValue);
    detectAndSwapLangs(sourceValue, langPair);
  }

  const handleApplySuggestion = () => {
    if (!translationResult.response.sourceCorrection) return;
    updateSourceText(translationResult.response.sourceCorrection, false);
  }

  const handleSpeak = () => {
    speak(sourceText);
  }

  return (
    <div className="relative">
      <Textarea
        className='flex-1 resize-none min-h-40 pb-8'
        ref={inputTextareaRef}
        value={sourceText}
        onChange={handleChange} />
      <div className="absolute bottom-1.5 right-1.5 left-1.5 flex justify-between items-center gap-2">
        {translationResult.response.sourceCorrection &&
          <Tooltip hint={<p>Did you mean: <b>{translationResult.response.sourceCorrection}</b></p>}>
            <span
              className="flex gap-0.5 items-center font-light whitespace-nowrap bg-background border pl-0.5 pr-1 rounded-md overflow-clip cursor-pointer"
              onClick={handleApplySuggestion}
            >
              <Lightbulb size={16} />
              {translationResult.response.sourceCorrection}
            </span>
          </Tooltip>
        }
        <div className="flex items-center gap-1.5 p-0.5 ml-auto">
          <Volume2 onClick={handleSpeak} className='cursor-pointer' />
          <MicIcon className='cursor-pointer' />
          {!settings.isAutoLanguageSwitchEnabled &&
            <ChevronsRightIcon onClick={translateCurrent} className='cursor-pointer' />
          }
        </div>
      </div>
    </div >
  )
}
