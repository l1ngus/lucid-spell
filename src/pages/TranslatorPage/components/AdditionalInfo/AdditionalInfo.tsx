import { useMemo, useState } from "react";
import useTranslation from "../../hooks/useTranslation";
import OtherTranslations from "./OtherTranslations";
import UsageExamples from "./UsageExamples";

export default () => {
  const { translationResult: { response: { translation, sourceCorrection } },
    sourceText,
    langPair
  } = useTranslation();

  const wordsCount = useMemo(() => {
    return sourceText.split(' ').length;
  }, [sourceText]);

  return (
    <div className="flex gap-5">
      <UsageExamples className="flex-1" />
      <OtherTranslations className="flex-1" />
    </div>
  )
}
