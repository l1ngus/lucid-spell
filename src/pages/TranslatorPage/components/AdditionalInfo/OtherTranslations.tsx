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

import { cn } from "@/lib/utils";
import type { ClassValue } from "clsx";
import TransList from "./TransList";
import useOtherTranslationsQuery from "../../hooks/queries/useOtherTranslationsQuery";
import useTranslation from "../../hooks/useTranslation";
import { Spinner } from "@/components/ui/spinner";
import useSettings from "@/app/hooks/useSettings";
import { useEffect, useState } from "react";
import { supportsOtherTranslations } from "../../lib/translationCapabilities";

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

  const isSupported = supportsOtherTranslations(settings.translationEngine);
  const shouldAutoFetch = settings.isAutoAltTransFetchEnabled && !!translationResult.response.translation;
  const isEnabled = isSupported && (shouldAutoFetch || isManualFetch);

  const { response, isFetching } = useOtherTranslationsQuery({
    engine: settings.translationEngine,
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

  if (!isSupported) return null;

  const parsedTranslations = response.otherTranslations.reduce<TrGroup[]>((acc, tr) => {
    const part = tr.part ?? '';
    const existing = acc.find(group => group.part === part);
    if (existing) existing.translations.push(tr.translation);
    else acc.push({ part, translations: [tr.translation] });
    return acc;
  }, []);

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
