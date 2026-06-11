import AppBarActions from "@/components/AppBar/AppBarActions";
import { Button } from "@/components/ui/button";
import { useDictionariesView } from "../hooks/useDictionariesView";
import { createDictionary } from "@/app/stores/dictionariesStore";

export default function () {
  const { currentView, openListView, openEditorView } = useDictionariesView();

  const handleNew = async () => {
    const newDict = await createDictionary({ name: 'New Dict' });
    openEditorView(newDict.meta.id);
  }

  return (
    <AppBarActions>
      <div className="grid items-center grid-cols-[1fr_auto_1fr]">
        <div />
        <h1 className="text-2xl font-bold">Dictionaries</h1>
        <div className="flex justify-end">
          {currentView === 'list' &&
            <Button className="font-bold rounded-lg" variant="outline" size="sm"
              onClick={handleNew}>
              New
            </Button>
          }

          {currentView === 'editor' &&
            <Button className="font-bold rounded-lg" variant="outline" size="sm"
              onClick={() => openListView()}>
              Back
            </Button>
          }
        </div>
      </div>
    </AppBarActions>
  )
}
