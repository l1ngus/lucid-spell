import { useEffect } from "react"
import { getCurrentWindow } from '@tauri-apps/api/window';

export default function () {
  useEffect(() => {
    // Показываем окно после монтирования компонента
    getCurrentWindow().show();
  }, []);
}
