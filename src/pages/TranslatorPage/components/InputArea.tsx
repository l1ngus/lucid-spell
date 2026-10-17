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

import { useRef, useEffect, type ChangeEvent } from 'react';
import { Textarea } from '@/components/ui/textarea';
import Tooltip from '@/components/ui/Tooltip';
import { ChevronsRightIcon, Volume2, Lightbulb } from 'lucide-react';
import isShortcutPressed from '@/app/helpers/isShortcutPressed';
import useTranslation from '../hooks/useTranslation';
import useSettings from '@/app/hooks/useSettings';
import useDetectAndSwap from '../hooks/useDetectAndSwap';
import useTextToSpeech from '../hooks/useTextToSpeech';
import useOcrPaste from '../hooks/useOcrPaste';


export default function () {
  const { settings } = useSettings();
  const detectAndSwapLangs = useDetectAndSwap();
  const { translateCurrent, updateSourceText, langPair, swapLangs, sourceText, translationResult } = useTranslation();
  const { speak } = useTextToSpeech();
  const inputTextareaRef = useRef<HTMLTextAreaElement>(null);
  const handlePaste = useOcrPaste(inputTextareaRef);

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
    if (settings.isAutoLanguageSwitchEnabled)
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
    if (isShortcutPressed(e, settings.clearShortcut)) {
      e.preventDefault();
      updateSourceText('');
      return;
    }

    if (isShortcutPressed(e, settings.swapLangsShortcut)) {
      e.preventDefault();
      swapLangs(true);
      return;
    }

    if (isShortcutPressed(e, settings.applyCorrectionShortcut)) {
      e.preventDefault();
      handleApplySuggestion();
      return;
    }
  }

  return (
    <div className="relative">
      <Textarea
        className='flex-1 resize-none min-h-40 pb-8'
        ref={inputTextareaRef}
        value={sourceText}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        onPaste={handlePaste}
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
          {/* <MicIcon className='cursor-pointer' /> */}
          {!settings.isAutoTranslateEnabled &&
            <ChevronsRightIcon onClick={translateCurrent} className='cursor-pointer' />
          }
        </div>
      </div>
    </div >
  )
}
