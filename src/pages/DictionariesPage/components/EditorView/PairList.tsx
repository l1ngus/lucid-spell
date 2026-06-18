import { useState } from "react"
import { addPair, updatePair, removePair } from "@/app/stores/dictionariesStore"
import type { DictionaryPair, TermPair } from "@/app/types/Dictionary"
import Pair from "./Pair"
import AddPairForm from "./AddPairForm"

interface PairListProps {
  dictId: string;
  pairs: DictionaryPair[];
  onChanged: () => void;
}

export default ({ dictId, pairs, onChanged }: PairListProps) => {
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editSource, setEditSource] = useState("");
  const [editTarget, setEditTarget] = useState("");

  const handleAdd = async (source: string, target: string) => {
    await addPair(dictId, { source, target });
    onChanged();
  };

  const startEditing = (pair: DictionaryPair) => {
    setEditingId(pair.id);
    setEditSource(pair.source);
    setEditTarget(pair.target);
  };

  const cancelEditing = () => {
    setEditingId(null);
    setEditSource("");
    setEditTarget("");
  };

  const handleUpdate = async (pairId: string) => {
    await updatePair(dictId, pairId, { source: editSource.trim(), target: editTarget.trim() });
    cancelEditing();
    onChanged();
  };

  const handleRemove = async (pairId: string) => {
    await removePair(dictId, pairId);
    onChanged();
  };

  const handleImportPairs = async (newPairs: TermPair[]) => {
    console.log(newPairs);
    const promises = newPairs.map(pair => addPair(dictId, pair));
    await Promise.all(promises);
    onChanged();
  }

  return (
    <div className="space-y-3">
      <AddPairForm onAdd={handleAdd} onImportPairs={handleImportPairs} />
      <div className="space-y-2">
        {pairs.map(pair => (
          <Pair
            key={pair.id}
            pair={pair}
            isEditing={editingId === pair.id}
            editSource={editSource}
            editTarget={editTarget}
            onEditSourceChange={setEditSource}
            onEditTargetChange={setEditTarget}
            onStartEdit={() => startEditing(pair)}
            onCancelEdit={cancelEditing}
            onSave={() => handleUpdate(pair.id)}
            onRemove={() => handleRemove(pair.id)}
          />
        ))}
        {pairs.length === 0 && (
          <p className="text-sm text-muted-foreground text-center">No pairs yet.</p>
        )}
      </div>
    </div>
  );
};
