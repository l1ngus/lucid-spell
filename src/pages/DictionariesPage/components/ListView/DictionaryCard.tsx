import { DictionaryMeta } from "@/app/types/Dictionary"
import { Button } from "@/components/ui/button";
import { useDictionariesView } from "../../hooks/useDictionariesView";

interface DictionaryCardProps {
  dictMeta: DictionaryMeta;
}

export default ({ dictMeta }: DictionaryCardProps) => {
  const { openEditorView } = useDictionariesView();

  return (
    <div className="flex justify-between py-1.5 px-3 border rounded-md items-center">
      <b>{dictMeta.name}</b>
      <div className="flex gap-1">
        <Button size="sm" variant="outline" onClick={() => openEditorView(dictMeta.id)}>Edit</Button>
        <Button size="sm" variant="outline">Open</Button>
      </div>
    </div>
  )
}
