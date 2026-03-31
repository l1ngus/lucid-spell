import { Badge } from "@/components/ui/badge";

export default ({ translations }: { translations: string[] }) => {
  return (
    <div className="ml-6 mt-1 mb-3 flex flex-wrap gap-1">
      {translations.map(tr => (
        <Badge key={tr} variant="secondary">{tr}</Badge>
      ))}
    </div>
  )
}
