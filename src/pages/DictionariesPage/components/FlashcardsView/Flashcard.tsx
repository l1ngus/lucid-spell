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

import { cn } from '@/lib/utils';
import { useState, useImperativeHandle, forwardRef } from 'react';
import SpeakButton from '@/components/SpeakButton';

export interface FlashcardRef {
  triggerScale: () => void;
}

interface FlashcardProps {
  sourceText: string;
  targetText: string;
  isFlipped: boolean;
  onFlip: () => void;
}

const Flashcard = forwardRef<FlashcardRef, FlashcardProps>(
  ({ isFlipped, onFlip, sourceText, targetText }, ref) => {
    const [isScaling, setIsScaling] = useState(false);
    const displayedText = isFlipped ? targetText : sourceText;

    useImperativeHandle(ref, () => ({
      triggerScale: () => {
        setIsScaling(true);
        setTimeout(() => {
          setIsScaling(false);
        }, 100);
      },
    }));

    return (
      <div
        onPointerDown={onFlip}
        className={cn(
          "relative w-60 h-36 mx-auto my-auto font-bold border select-none rounded-md bg-input/40 cursor-pointer transition-transform duration-200",
          isScaling ? 'scale-95' : 'scale-100'
        )}
      >
        <div className="flex h-full items-start justify-center px-3 py-1 overflow-y-auto">
          <span className="max-w-full my-auto px-6 wrap-break-word">
            {displayedText}
          </span>
        </div>
        <SpeakButton text={displayedText} className="absolute top-1.5 right-1.5 z-10" />
      </div>
    );
  }
);

export default Flashcard;
