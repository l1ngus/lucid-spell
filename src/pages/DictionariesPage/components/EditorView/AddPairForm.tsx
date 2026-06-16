import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Plus } from "lucide-react"
import { FileDown } from "lucide-react"
import ImportPairsDialog from "./ImportPairsDialog"

interface AddPairFormProps {
  onAdd: (source: string, target: string) => void;
}

export default ({ onAdd }: AddPairFormProps) => {
  const [source, setSource] = useState("");
  const [target, setTarget] = useState("");

  const handleAdd = () => {
    if (!source.trim() || !target.trim()) return;
    onAdd(source.trim(), target.trim());
    setSource("");
    setTarget("");
  };

  return (
    <div className="grid grid-cols-[1fr_auto_1fr] items-center gap-2">
      <ImportPairsDialog>
        <Button size="icon-sm" variant="outline" >
          <FileDown />
        </Button>
      </ImportPairsDialog>
      <div className="flex gap-2 items-center flex-row">
        <Input
          placeholder="Source text"
          value={source}
          onChange={e => setSource(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && handleAdd()}
          className="flex-1 h-8"
        />
        <span className="text-muted-foreground shrink-0">→</span>
        <Input
          placeholder="Target text"
          value={target}
          onChange={e => setTarget(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && handleAdd()}
          className="flex-1 h-8"
        />
      </div>
      <Button
        size="icon-sm"
        onClick={handleAdd}
        disabled={!source.trim() || !target.trim()}
      >
        <Plus className="size-4" />
      </Button>
    </div >
  )
}
