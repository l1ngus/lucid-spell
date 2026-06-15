import { useEffect, useState } from "react"
import { type Dictionary } from "@/app/types/Dictionary";
import { getAllDictionaries } from "@/app/stores/dictionariesStore";
import sortDictionaries from "@/app/helpers/sortDictionaries";
import DictionaryCard from "./DictionaryCard";



export default () => {
  const [dictionaries, setDictionaries] = useState<Dictionary[]>([]);

  useEffect(() => {
    getAllDictionaries().then(dicts => {
      setDictionaries(sortDictionaries(dicts));
    });
  }, []);

  return (
    <div className="flex flex-col gap-2 max-w-xl w-[75%] mt-4 mx-auto">
      {dictionaries.map(dict => (
        <DictionaryCard key={dict.meta.id} dictMeta={dict.meta} />
      ))}
    </div >
  )
}
