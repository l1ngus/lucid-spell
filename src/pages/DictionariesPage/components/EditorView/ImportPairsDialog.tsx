import { useState, type ReactNode } from "react"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"

interface ImportPairsDialogProps {
  children: ReactNode
}

export default ({ children }: ImportPairsDialogProps) => {
  const [open, setOpen] = useState(false);

  const handleOpenChange = (open: boolean) => {
    setOpen(open);
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        {children}
      </DialogTrigger>
      <DialogContent aria-description={undefined} className="flex gap-0 flex-col max-h-80 min-w-60 max-w-120 sm:max-w-120 h-[calc(100dvh-2rem)] w-[calc(100vw-2rem)]">
        <DialogHeader className="h-fit">
          <DialogTitle>Import pairs</DialogTitle>
        </DialogHeader>
        <div className="">
        </div>
      </DialogContent>
    </Dialog>
  )
}

