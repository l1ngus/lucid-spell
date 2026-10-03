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
          "flex justify-center cursor-pointer items-start px-3 py-1 w-60 h-36 mx-auto my-auto font-bold border select-none rounded-md bg-input/40 transition-transform duration-200 overflow-y-auto",
          isScaling ? 'scale-95' : 'scale-100'
        )}
      >
        <span className="max-w-full my-auto wrap-break-word">
          {isFlipped ? targetText : sourceText}
        </span>
      </div>
    );
  }
);

export default Flashcard;
