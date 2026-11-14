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

import type { MouseEvent, PointerEvent } from 'react';
import { Volume2 } from 'lucide-react';
import useTextToSpeech from '@/app/hooks/useTextToSpeech';
import { Button } from '@/components/ui/button';
import { Spinner } from '@/components/ui/spinner';
import { cn } from '@/lib/utils';

interface SpeakButtonProps {
  text: string;
  className?: string;
}

const SpeakButton = ({ text, className }: SpeakButtonProps) => {
  const { speak, isSpeaking } = useTextToSpeech();

  // The button may live inside a clickable container (e.g. a flashcard),
  // so it must not let pointer events reach the parent.
  const handlePointerDown = (e: PointerEvent<HTMLButtonElement>) => {
    e.stopPropagation();
  };

  const handleClick = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    if (isSpeaking) return;
    void speak(text);
  };

  return (
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
      className={cn('cursor-pointer', className)}
      aria-label="Pronounce text"
      title="Pronounce text"
      aria-busy={isSpeaking}
      onPointerDown={handlePointerDown}
      onClick={handleClick}
    >
      {isSpeaking ? <Spinner /> : <Volume2 />}
    </Button>
  );
};

export default SpeakButton;
