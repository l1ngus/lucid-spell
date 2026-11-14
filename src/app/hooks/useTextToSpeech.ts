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

import { useCallback, useRef, useState } from 'react';
import { commands } from '@/bindings';
import useSettings from '@/app/hooks/useSettings';

export default function useTextToSpeech() {
  const { settings } = useSettings();
  const [isSpeaking, setIsSpeaking] = useState(false);
  const isSpeakingRef = useRef(false);

  const speak = useCallback(async (text: string) => {
    const trimmedText = text.trim();
    if (!trimmedText || isSpeakingRef.current) return;

    isSpeakingRef.current = true;
    setIsSpeaking(true);

    try {
      const res = await commands.speak(trimmedText, settings.voice);
      if (res.status === 'error') {
        console.error(res.error);
      }
    } catch (error) {
      console.error(error);
    } finally {
      isSpeakingRef.current = false;
      setIsSpeaking(false);
    }
  }, [settings.voice]);

  return { speak, isSpeaking };
}
