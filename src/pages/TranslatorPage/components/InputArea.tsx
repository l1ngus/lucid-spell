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
  const { translateCurrent, updateSourceText, langPair, swapLangs, sourceText, translationResult
  } = useTranslation();
  const { speak } = useTextToSpeech();
  const inputTextareaRef = useRef<HTMLTextAreaElement>(null);
  const pressedKeysRef = useRef<Set<string>>(new Set());

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

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    pressedKeysRef.current.add(e.key.toLowerCase());

    if (settings.clearShortcut.every(key => pressedKeysRef.current.has(key.toLowerCase())))
      updateSourceText('');
    if (settings.swapLangsShortcut.every(key => pressedKeysRef.current.has(key.toLowerCase())))
      swapLangs(true);
    if (settings.applyCorrectionShortcut.every(key => pressedKeysRef.current.has(key.toLowerCase())))
      handleApplySuggestion() // this function checks if sourceCorrection is empty
  }

  const handleKeyUp = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    pressedKeysRef.current.delete(e.key.toLowerCase());
  }

  return (
    <div className="relative">
      <Textarea
        className='flex-1 resize-none min-h-40 pb-8'
        ref={inputTextareaRef}
        value={sourceText}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        onKeyUp={handleKeyUp}
      />
      <div className="absolute bottom-1.5 right-1.5 left-1.5 flex justify-between items-center gap-2">
        {translationResult.response.sourceCorrection &&
          <Tooltip className="min-w-0 shrink" hint={<p>Did you mean: <b>{translationResult.response.sourceCorrection}</b></p>}>
            <span
              className="flex gap-0.5 min-w-0 max-w-full items-center font-light bg-background border pl-0.5 pr-1 rounded-md cursor-pointer "
              onClick={handleApplySuggestion}
            >
              <Lightbulb size={16} className='shrink-0' />
              <span className="truncate">
                {translationResult.response.sourceCorrection}
              </span>

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
