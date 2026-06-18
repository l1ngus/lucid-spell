import { useRef, useState, type ReactNode } from "react"
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Button } from "@/components/ui/button";
import { INNER_SEPARATORS, OUTER_SEPARATORS, DEFAULT_INNER, DEFAULT_OUTER } from "../../consts/Separators";
import type { InnerSeparator, OuterSeparator, SepPair } from "../../types/separators";
import getPairsPreview from "../../helpers/getPairsPreview";
import { useDictionariesView } from "../../hooks/useDictionariesView";
import type { TermPair } from "@/app/types/Dictionary";

interface ImportPairsDialogProps {
  children: ReactNode;
  onImportPairs: (newPairs: TermPair[]) => void;
}

const ImportPairsDialog = ({ children, onImportPairs }: ImportPairsDialogProps) => {
  const { dictId } = useDictionariesView();
  const [open, setOpen] = useState(false);
  const [separators, setSeparators] = useState<SepPair>({
    inner: DEFAULT_INNER,
    outer: DEFAULT_OUTER
  });
  const [areaPlaceholder, setAreaPlaceholder] = useState(getPairsPreview(separators));
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const handleOpenChange = (open: boolean) => {
    setOpen(open);
  }

  const handleSeparatorChange = (sepPair: Partial<SepPair>) => {
    setSeparators(prev => {
      const newSepPair = { ...prev, ...sepPair };
      setAreaPlaceholder(getPairsPreview(newSepPair));
      return newSepPair;
    });
  }

  const handleImport = async () => {
    if (!textareaRef.current || !dictId) return
    const strPairs: string[] = textareaRef.current.value.split(OUTER_SEPARATORS[separators.outer].sep);
    const newPairs: TermPair[] = [];
    for (const strPair of strPairs) {
      const pairEntries = strPair.split(INNER_SEPARATORS[separators.inner].sep, 2);
      if (pairEntries.length !== 2) continue;
      newPairs.push({
        source: pairEntries[0].trim(),
        target: pairEntries[1].trim()
      })
    }
    onImportPairs(newPairs);
    setOpen(false);
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        {children}
      </DialogTrigger>
      <DialogContent aria-describedby={undefined} className="p-2 flex gap-1 flex-row max-h-80 max-w-120 sm:max-w-120 h-[calc(100dvh-2rem)] w-[calc(100vw-2rem)]">
        <Textarea ref={textareaRef} placeholder={areaPlaceholder} className="flex-1 resize-none" />
        <div className="flex gap-2 flex-col flex-1 mt-2 ml-2">
          <DialogTitle className="mb-1">Import pairs</DialogTitle>

          {/* INNER SEPARATOR */}
          <div className="grid w-full max-w-xs items-center gap-1.5">
            <Label htmlFor="inner-separator-select">Inner separator</Label>
            <Select defaultValue={DEFAULT_INNER} onValueChange={val => handleSeparatorChange({ inner: val as InnerSeparator })} >
              <SelectTrigger size="sm" id="inner-separator-select">
                <SelectValue placeholder="Inner separator" />
              </SelectTrigger>
              <SelectContent>
                {Object.entries(INNER_SEPARATORS).map(([value, obj]) => (
                  <SelectItem key={value} value={value}>
                    {obj.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* OUTER SEPARATOR */}
          <div className="grid w-full max-w-xs items-center gap-1.5">
            <Label htmlFor="outer-separator-select">Inner separator</Label>
            <Select defaultValue={DEFAULT_OUTER} onValueChange={val => handleSeparatorChange({ outer: val as OuterSeparator })} >
              <SelectTrigger size="sm" id="outer-separator-select">
                <SelectValue placeholder="Outer separator" />
              </SelectTrigger>
              <SelectContent>
                {Object.entries(OUTER_SEPARATORS).map(([value, obj]) => (
                  <SelectItem key={value} value={value}>
                    {obj.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <Button onClick={handleImport} size="sm" variant="outline" className="mt-auto">
            Import
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}

export default ImportPairsDialog;
