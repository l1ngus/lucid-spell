import { APP_BAR_ACTIONS_ID } from "./consts";
import Navigation from "../Navigation/Navigation";
import TitleBarActions from "./TitleBarActions";
import { getCurrentWindow } from "@tauri-apps/api/window";
import './AppBar.scss';

export default function () {

  const handleWindowDrag = (e: React.PointerEvent<HTMLElement>) => {
    if (e.button !== 0 || e.pointerType !== 'mouse') return;
    if ((e.target as HTMLElement).closest('button, svg, [data-no-drag]')) return;
    void getCurrentWindow().startDragging();
  }

  return (
    <header className="app-bar"
      onPointerDown={handleWindowDrag}>
      <Navigation />
      <div className="w-full h-full" id={APP_BAR_ACTIONS_ID} />
      <TitleBarActions />
    </header>
  )
}
