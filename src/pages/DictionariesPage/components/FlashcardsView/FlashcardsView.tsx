import { useEffect, useState } from "react";
import type { DictionaryPair } from "@/app/types/Dictionary";
import { useDictionariesView } from "../../hooks/useDictionariesView";
import Flashcard from "./Flashcard";
import FlashcardNavBar from "./FlashcardNavBar";
import { getDictionary } from "@/app/stores/dictionariesStore";
import shuffleArray from "@/app/helpers/shuffleArray";

const FlashcardsView = () => {
  const { dictId } = useDictionariesView();
  const [pairs, setPairs] = useState<DictionaryPair[] | null>(null)
  const [currentPairInd, setCurrentPairInd] = useState(0);

  useEffect(() => {
    if (!dictId) return;
    getDictionary(dictId)
      .then(loadedDict => setPairs(() => {
        if (!loadedDict) return null;
        return shuffleArray(loadedDict.pairs);
      }));
  }, [dictId]);

  if (!pairs) {
    return <div />;
  }

  const handlePrev = () => {
    setCurrentPairInd(prev => prev - 1);
  }

  const handleNext = () => {
    setCurrentPairInd(prev => prev + 1);
  }

  return (
    <div className="flex flex-1 flex-col ">
      <Flashcard sourceText={pairs[currentPairInd].source} targetText={pairs[currentPairInd].target} />
      <FlashcardNavBar currentPairInd={currentPairInd} pairsAmount={pairs.length} onPrev={handlePrev} onNext={handleNext} />
    </div>
  )
}

export default FlashcardsView;
