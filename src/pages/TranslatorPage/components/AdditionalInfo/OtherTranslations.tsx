import { cn } from "@/lib/utils";
import type { ClassValue } from "clsx";
import TransList from "./TransList";
import useOtherTranslationsQuery from "../../hooks/queries/useOtherTranslationsQuery";
import useTranslation from "../../hooks/useTranslation";
import type { TranslateOthersResponse, TranslateOthersWithPartsResponse } from "../../types/TranslateResponse";
import { Spinner } from "@/components/ui/spinner";
import useSettings from "@/app/hooks/useSettings";
import { useEffect, useState } from "react";

interface OtherTranslationsProps {
  className?: ClassValue;
}

interface TrGroup {
  part: string;
  translations: string[];
}

export default ({ className }: OtherTranslationsProps) => {
  const { settings } = useSettings();
  const { translationResult, langPair, sourceText } = useTranslation();
  const [isManualFetch, setIsManualFetch] = useState(false);

  const shouldAutoFetch = settings.isAutoAltTransFetchEnabled && !!translationResult.response.translation;
  const isEnabled = shouldAutoFetch || isManualFetch;

  const { response, isFetching } = useOtherTranslationsQuery({
    sourceText,
    translatedText: translationResult.response.translation ?? '',
    sourceLang: langPair.source,
    targetLang: langPair.target,
    maxSourceLength: settings.isAutoAltTransFetchEnabled ? 50 : undefined,
    isEnabled
  });

  useEffect(() => {
    if (!settings.isAutoAltTransFetchEnabled) setIsManualFetch(false);
  }, [sourceText]);

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
      {settings.isAutoAltTransFetchEnabled
        ? <p className="text-center ">Other translations</p>
        : <p onClick={() => setIsManualFetch(true)} className="m-auto w-fit px-2 rounded-md cursor-pointer bg-accent select-none">Other translations</p>
      }
      {isEnabled && !isFetching && parsedTranslations.map(group => (
        <div key={group.part}>
          <p>{group.part}</p>
          <TransList translations={group.translations} />
        </div>
      ))}
      {isFetching && <Spinner className="m-auto size-8" />}
    </div>
  )
}
