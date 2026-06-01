import useSettings from "@/app/hooks/useSettings";
import AppBarActions from "@/components/AppBar/AppBarActions";
import { Button } from "@/components/ui/button";

export default function () {
  const { isSaved, saveSettings } = useSettings();

  return (
    <AppBarActions>
      <div className="grid items-center grid-cols-[1fr_auto_1fr]">
        <div />
        <h1 className="text-2xl font-bold">Settings</h1>
        <div className="flex justify-end">
          <Button className="font-bold rounded-lg" variant="outline" size="sm"
            disabled={isSaved} onClick={saveSettings}>
            Save
          </Button>
        </div>
      </div>
    </AppBarActions>
  )
}
