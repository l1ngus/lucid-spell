import type { DictionaryPair } from "@/app/types/Dictionary"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Pencil, X, Check, Trash2 } from "lucide-react"
import Tooltip from '@/components/ui/Tooltip'

interface PairProps {
  pair: DictionaryPair;
  isEditing: boolean;
  editSource: string;
  editTarget: string;
  onEditSourceChange: (value: string) => void;
  onEditTargetChange: (value: string) => void;
  onStartEdit: () => void;
  onCancelEdit: () => void;
  onSave: () => void;
  onRemove: () => void;
}

function DisplayCell({ text, lang }: { text: string; lang?: string | null }) {
  const cls = "flex-1 font-mono text-sm truncate border rounded-md px-3 py-1 h-8"
  const hint = lang ? lang[0].toUpperCase() + lang.slice(1) : undefined
  if (lang) return <Tooltip hint={hint} className={cls}>{text}</Tooltip>
  return <span className={cls}>{text}</span>
}

export default ({
  pair,
  isEditing,
  editSource,
  editTarget,
  onEditSourceChange,
  onEditTargetChange,
  onStartEdit,
  onCancelEdit,
  onSave,
  onRemove,
}: PairProps) => {
  if (isEditing) {
    return (
      <div className="flex items-center gap-2">
        <Input
          value={editSource}
          onChange={e => onEditSourceChange(e.target.value)}
          className="flex-1"
        />
        <span className="text-muted-foreground shrink-0">→</span>
        <Input
          value={editTarget}
          onChange={e => onEditTargetChange(e.target.value)}
          className="flex-1"
        />
        <Button size="icon-sm" variant="ghost" onClick={onSave}>
          <Check className="size-4" />
        </Button>
        <Button size="icon-sm" variant="ghost" onClick={onCancelEdit}>
          <X className="size-4" />
        </Button>
      </div>
    )
  }

  return (
    <div className="flex items-center gap-2">
      <Button size="icon-sm" variant="outline" onClick={onRemove} className="h-8 w-8">
        <Trash2 className="size-4" />
      </Button>

      <DisplayCell text={pair.source} lang={pair.sourceLang} />
      <span className="text-muted-foreground shrink-0 flex items-center justify-center h-8 text-lg leading-none">
        →
      </span>
      <DisplayCell text={pair.target} lang={pair.targetLang} />

      <Button size="icon-sm" variant="outline" onClick={onStartEdit} className="h-8 w-8">
        <Pencil className="size-4" />
      </Button>
    </div>)
}
