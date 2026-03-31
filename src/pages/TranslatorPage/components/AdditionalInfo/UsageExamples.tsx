import { cn } from "@/lib/utils";
import type { ClassValue } from "clsx"

interface UsageExamplesProps {
  className?: ClassValue;
}



export default ({ className }: UsageExamplesProps) => {
  return (
    <div className={cn(className)}>
      <p className="text-center">Usage examples</p>
    </div>
  )
}
