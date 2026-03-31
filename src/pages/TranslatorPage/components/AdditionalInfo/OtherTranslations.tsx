import { cn } from "@/lib/utils";
import type { ClassValue } from "clsx";
import TransList from "./TransList";

interface OtherTranslationsProps {
  className?: ClassValue;
}

interface TrGroup {
  part: string;
  translations: string[];
}


export default ({ className }: OtherTranslationsProps) => {
  const response = { "otherTranslations": [{ "part": "прил.", "translation": "поздний" }, { "part": "нареч.", "translation": "поздно" }, { "part": "прил.", "translation": "запоздалый" }, { "part": "прил.", "translation": "покойный" }, { "part": "прил.", "translation": "бывший" }, { "part": "нареч.", "translation": "недавно" }, { "part": "прил.", "translation": "последний" }] };

  const parsedTranslations = response.otherTranslations.reduce<TrGroup[]>((acc, tr) => {
    const existing = acc.find(group => group.part === tr.part);
    if (existing) existing.translations.push(tr.translation);
    else acc.push({ part: tr.part, translations: [tr.translation] });
    return acc;
  }, []);

  return (
    <div className={cn(className)}>
      <p className="text-center ">Other translations</p>

      {parsedTranslations.map(group => (
        <div>
          <p>{group.part}</p>
          <TransList translations={group.translations} />
        </div>
      ))}

      {/* {sourceTextVolume === 'expression' && <TransList translations={parsedTranslations} />} */}
    </div>
  )
}
