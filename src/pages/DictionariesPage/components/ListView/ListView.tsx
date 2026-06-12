import { useEffect, useState } from "react"
import { type Dictionary } from "@/app/types/Dictionary";
import { getAllDictionaries } from "@/app/stores/dictionariesStore";
import DictionaryCard from "./DictionaryCard";



export default () => {
  const [dictionaries, setDictionaries] = useState<Dictionary[]>([]);

  useEffect(() => {
    getAllDictionaries().then(dicts => {
      const sorted = [...dicts].sort((a, b) =>
        b.meta.updatedAt - a.meta.updatedAt
      );
      const favIndex = sorted.findIndex(d => d.meta.isFavorites);
      if (favIndex !== -1) {
        const [fav] = sorted.splice(favIndex, 1);
        sorted.unshift(fav);
      }
      setDictionaries(sorted);
    });
  }, []);

  return (
    <div className="flex flex-col gap-2 max-w-xl w-[75%] mt-4 mx-auto">
      {dictionaries.map(dict => (
        <DictionaryCard dictMeta={dict.meta} />
      ))}
    </div >
  )
}
