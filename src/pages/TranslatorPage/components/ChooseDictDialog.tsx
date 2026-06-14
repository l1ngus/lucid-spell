import { useEffect, useState, type ReactNode } from "react"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { type Dictionary } from "@/app/types/Dictionary";
import { getAllDictionaries } from "@/app/stores/dictionariesStore";
import sortDictionaries from "@/app/helpers/sortDictionaries";

interface ChooseDictDialogProps {
  children: ReactNode;
  onDictSelect: (dictId: string) => void;
  checkWhetherOpen?: () => boolean;
}

export default ({ children, onDictSelect, checkWhetherOpen }: ChooseDictDialogProps) => {
  const [dicts, setDicts] = useState<Dictionary[]>([]);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    getAllDictionaries()
      .then(loadedDicts => setDicts(sortDictionaries(loadedDicts)));
  }, []);

  const handleSelect = (dictId: string) => {
    onDictSelect(dictId);
    setOpen(false);
  }

  const handleOpenChange = (open: boolean) => {
    if (open && checkWhetherOpen && !checkWhetherOpen()) return;
    setOpen(open);
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        {children}
      </DialogTrigger>
      <DialogContent className="flex gap-0 flex-col max-h-100 min-w-60 max-w-80 sm:max-w-80 h-[calc(100dvh-2rem)] w-[calc(100vw-14rem)]">
        <DialogHeader className="h-fit">
          <DialogTitle>Choose dictonary</DialogTitle>
        </DialogHeader>
        <div className="flex gap-2 flex-col h-full -mx-4 no-scrollbar max-h-[70vh] overflow-y-auto px-5 py-2">
          {dicts.map(dict => (
            <div onClick={() => handleSelect(dict.meta.id)} className="py-1 px-2 border rounded-md cursor-pointer hover:scale-105 z-40 transition-transform duration-150">
              {dict.meta.name}
            </div>
          ))}
        </div>
      </DialogContent>
    </Dialog>
  )
}

