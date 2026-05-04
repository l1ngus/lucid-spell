import { cn } from "@/lib/utils";
import type { ClassValue } from "clsx";
import TransList from "./TransList";
import useOtherTranslationsQuery from "../../hooks/queries/useOtherTranslationsQuery";
import useTranslation from "../../hooks/useTranslation";
import type { TranslateOthersResponse, TranslateOthersWithPartsResponse } from "../../types/TranslateResponse";

interface OtherTranslationsProps {
  className?: ClassValue;
}

interface TrGroup {
  part: string;
  translations: string[];
}


export default ({ className }: OtherTranslationsProps) => {
  const { translationResult, langPair, sourceText } = useTranslation();
  const { response } = useOtherTranslationsQuery({
    sourceText,
    translatedText: translationResult.response.translation ?? '',
    sourceLang: langPair.source,
    targetLang: langPair.target
  });

  const isWithParts = response.otherTranslations.length > 0
    && typeof (response.otherTranslations[0]) !== 'string'
    && 'part' in response.otherTranslations[0];

  let parsedTranslations: TrGroup[];
  if (isWithParts) {
    const trs = response.otherTranslations as TranslateOthersWithPartsResponse['otherTranslations'];
    parsedTranslations = trs.reduce<TrGroup[]>((acc, tr) => {
      const existing = acc.find(group => group.part === tr.part);
      if (existing) existing.translations.push(tr.translation);
      else acc.push({ part: tr.part, translations: [tr.translation] });
      return acc;
    }, []);
  } else {
    const trs = response.otherTranslations as TranslateOthersResponse['otherTranslations'];
    parsedTranslations = [{ part: '', translations: trs }];
  }

  return (
    <div className={cn(className)}>
      <p className="text-center ">Other translations</p>
      {parsedTranslations.map(group => (
        <div key={group.part}>
          <p>{group.part}</p>
          <TransList translations={group.translations} />
        </div>
      ))}
    </div>
  )
}
