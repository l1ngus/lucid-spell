import { useState, useEffect, useCallback } from "react"
import { useDictionariesView } from "../../hooks/useDictionariesView"
import { getDictionary, deleteDictionary, updateDictionaryMeta } from "@/app/stores/dictionariesStore"
import type { Dictionary } from "@/app/types/Dictionary"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { Separator } from "@/components/ui/separator"
import { Trash2, Save } from "lucide-react"
import PairList from "./PairList"

export default () => {
  const { dictId, openListView } = useDictionariesView();
  const [dict, setDict] = useState<Dictionary | null>(null);
  const [loading, setLoading] = useState(true);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [sourceLang, setSourceLang] = useState("");
  const [targetLang, setTargetLang] = useState("");

  const reloadDict = useCallback(async () => {
    if (!dictId) return;
    const d = await getDictionary(dictId);
    setDict(d ?? null);
  }, [dictId]);

  useEffect(() => {
    if (!dictId) return;
    setLoading(true);
    getDictionary(dictId).then(d => {
      setDict(d ?? null);
      if (d) {
        setName(d.meta.name);
        setDescription(d.meta.description ?? "");
        setSourceLang(d.meta.sourceLang ?? "");
        setTargetLang(d.meta.targetLang ?? "");
      }
      setLoading(false);
    });
  }, [dictId]);

  const handleSave = async () => {
    if (!dictId) return;
    await updateDictionaryMeta(dictId, { name, description, sourceLang, targetLang });
    await reloadDict();
  };

  const handleDelete = async () => {
    if (!dictId) return;
    await deleteDictionary(dictId);
    openListView();
  };

  if (loading) {
    return <div className="p-4 text-muted-foreground">Loading...</div>;
  }

  if (!dict) {
    return <div className="p-4 text-muted-foreground">Dictionary not found.</div>;
  }

  const hasChanges = name !== dict.meta.name
    || description !== (dict.meta.description ?? "")
    || (!dict.meta.isFavorites && (
      sourceLang !== (dict.meta.sourceLang ?? "")
      || targetLang !== (dict.meta.targetLang ?? "")
    ));

  return (
    <div className="p-4 space-y-6 max-w-2xl mx-[5%]">
      <div className="space-y-4">
        <div className="space-y-3">
          <div className="space-y-1">
            <Label htmlFor="name">Name</Label>
            <Input disabled={dict.meta.isFavorites} id="name" value={name} onChange={e => setName(e.target.value)} />
          </div>
          <div className="space-y-1">
            <Label htmlFor="description">Description</Label>
            <Textarea disabled={dict.meta.isFavorites} className="min-h-9 py-0.5" id="description" value={description} onChange={e => setDescription(e.target.value)} />
          </div>
          {!dict.meta.isFavorites &&
            <div className="grid grid-cols-2 gap-3">
              <div className="space-y-1">
                <Label htmlFor="sourceLang">Source Language</Label>
                <Input id="sourceLang" value={sourceLang} onChange={e => setSourceLang(e.target.value)} />
              </div>
              <div className="space-y-1">
                <Label htmlFor="targetLang">Target Language</Label>
                <Input id="targetLang" value={targetLang} onChange={e => setTargetLang(e.target.value)} />
              </div>
            </div>
          }
          {!dict.meta.isFavorites &&
            <div className="flex justify-end">
              <Button size="sm" onClick={handleSave} disabled={!hasChanges}>
                <Save className="size-4" />
                Save Changes
              </Button>
            </div>
          }
        </div>
      </div>

      <Separator />

      <PairList dictId={dictId!} pairs={dict.pairs} onChanged={reloadDict} />

      {!dict.meta.isFavorites && <Separator />}

      {!dict.meta.isFavorites &&
        <div className="flex justify-center">
          <Button className="" size="sm" variant="destructive" onClick={handleDelete}>
            <Trash2 className="size-4" />
            Delete Dictionary
          </Button>
        </div>
      }
    </div >
  );
}
