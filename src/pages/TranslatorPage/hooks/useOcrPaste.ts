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

import { useCallback, useRef, type ClipboardEvent, type RefObject } from 'react';
import useSettings from '@/app/hooks/useSettings';
import useTranslation from './useTranslation';
import useImageOcr from './useImageOcr';
import useDetectAndSwap from './useDetectAndSwap';

function getImageFromClipboard(clipboard: DataTransfer): File | null {
  for (const item of Array.from(clipboard.items)) {
    if (item.kind !== 'file' || !item.type.startsWith('image/')) continue;
    const file = item.getAsFile();
    if (file) return file;
  }

  return Array.from(clipboard.files).find(file => file.type.startsWith('image/')) ?? null;
}

function getErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export default function useOcrPaste(textareaRef: RefObject<HTMLTextAreaElement | null>) {
  const { langPair, updateSourceText, setOcrError } = useTranslation();
  const { settings } = useSettings();
  const recognizeImage = useImageOcr();
  const detectAndSwapLangs = useDetectAndSwap();
  const requestIdRef = useRef(0);

  return useCallback(async (event: ClipboardEvent<HTMLTextAreaElement>) => {
    const image = getImageFromClipboard(event.clipboardData);
    if (!image) return;

    event.preventDefault();
    setOcrError(null);
    const requestId = ++requestIdRef.current;
    const lang = langPair.source === 'auto' ? 'eng' : langPair.source;

    try {
      const recognizedText = await recognizeImage(image, lang);
      if (requestId !== requestIdRef.current) return;

      if (!recognizedText.trim()) {
        setOcrError('No text was found in the image.');
        return;
      }

      const textarea = textareaRef.current;
      const currentText = textarea?.value ?? '';
      const selectionStart = textarea?.selectionStart ?? currentText.length;
      const selectionEnd = textarea?.selectionEnd ?? selectionStart;
      const nextText = currentText.slice(0, selectionStart) + recognizedText + currentText.slice(selectionEnd);
      updateSourceText(nextText);
      if (settings.isAutoLanguageSwitchEnabled) detectAndSwapLangs(nextText, langPair);

      if (textarea && document.activeElement === textarea) {
        const cursorPosition = selectionStart + recognizedText.length;
        requestAnimationFrame(() => {
          if (document.activeElement === textarea) {
            textarea.setSelectionRange(cursorPosition, cursorPosition);
          }
        });
      }
    } catch (error) {
      if (requestId === requestIdRef.current) setOcrError(getErrorMessage(error));
    }
  }, [detectAndSwapLangs, langPair, recognizeImage, setOcrError, settings.isAutoLanguageSwitchEnabled, textareaRef, updateSourceText]);
}
