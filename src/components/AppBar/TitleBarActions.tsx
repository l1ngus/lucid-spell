import { getCurrentWindow } from "@tauri-apps/api/window";
import { XIcon, MaximizeIcon, MinusIcon } from "lucide-react";

const appWindow = getCurrentWindow();

const TitleBarActions = (): React.ReactNode => {
  return (
    <div className="flex gap-1 justify-end items-center">
      <div className="bg-accent/40 rounded-md p-1 cursor-pointer hover:bg-accent/80 transition-colors duration-150">
        <MinusIcon size={20} onClick={() => appWindow.minimize()} className="text-foreground/80 hover:text-foreground" />
      </div>
      <div className="bg-accent/40 rounded-md p-1 cursor-pointer hover:bg-accent/80 transition-colors duration-150">
        <MaximizeIcon size={20} onClick={() => appWindow.toggleMaximize()} className="text-foreground/80 hover:text-foreground" />
      </div>
      <div className="bg-destructive/20 rounded-md p-1 cursor-pointer hover:bg-destructive/50 transition-colors duration-150">
        <XIcon size={20} onClick={() => appWindow.close()} />
      </div>
    </div>
  )
}

export default TitleBarActions;
