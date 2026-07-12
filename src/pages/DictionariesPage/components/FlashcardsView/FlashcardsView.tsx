import { useEffect, useState, useRef } from "react";
import type { DictionaryPair } from "@/app/types/Dictionary";
import { useDictionariesView } from "../../hooks/useDictionariesView";
import Flashcard, { type FlashcardRef } from "./Flashcard";
import FlashcardNavBar from "./FlashcardNavBar";
import { getDictionary } from "@/app/stores/dictionariesStore";
import shuffleArray from "@/app/helpers/shuffleArray";
import isShortcutPressed from "@/app/helpers/isShortcutPressed";
import useSettings from "@/app/hooks/useSettings";

const FlashcardsView = () => {
  const { settings } = useSettings();
  const { dictId } = useDictionariesView();
  const [pairs, setPairs] = useState<DictionaryPair[] | null>(null)
  const [currentPairInd, setCurrentPairInd] = useState(0);
  const [isFlashcardFlipped, setIsFlashcardFlipped] = useState(false);
  const flashcardRef = useRef<FlashcardRef>(null);

  useEffect(() => {
    if (!dictId) return;
    getDictionary(dictId)
      .then(loadedDict => setPairs(() => {
        if (!loadedDict) return null;
        return shuffleArray(loadedDict.pairs);
      }));
  }, [dictId]);

  const handleFlip = () => {
    setIsFlashcardFlipped(prev => !prev);
    flashcardRef.current?.triggerScale();
  }

  const handlePrev = () => {
    setCurrentPairInd(prev => {
      if (prev - 1 >= 0) {
        setIsFlashcardFlipped(false);
        return prev - 1;
      }
      return prev;
    })
  }

  const handleNext = () => {
    setCurrentPairInd(prev => {
      if (pairs && prev + 1 < pairs.length) {
        setIsFlashcardFlipped(false);
        return prev + 1;
      }
      return prev;
    })
  }

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (isShortcutPressed(e, settings.flipFlashcardShortcut)) {
        e.preventDefault();
        handleFlip();
      }
      if (isShortcutPressed(e, settings.prevFlashcardShortcut)) {
        handlePrev();
      }
      if (isShortcutPressed(e, settings.nextFlashcardShortcut)) {
        handleNext();
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [settings, handlePrev, handleNext]);



  if (!pairs)
    return <div />;

  return (
    <div className="flex flex-1 flex-col ">
      <Flashcard ref={flashcardRef} isFlipped={isFlashcardFlipped} onFlip={handleFlip} sourceText={pairs[currentPairInd].source} targetText={pairs[currentPairInd].target} />
      <FlashcardNavBar currentPairInd={currentPairInd} pairsAmount={pairs.length} onPrev={handlePrev} onNext={handleNext} />
    </div>
  )
}

export default FlashcardsView;
