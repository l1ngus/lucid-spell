import { useEffect } from "react"
import { getCurrentWindow } from '@tauri-apps/api/window';

export default function () {
  useEffect(() => {
    // Показываем окно после монтирования компонента
    getCurrentWindow().show();
  }, []);

  // Если упадет синтаксис или не загрузится JS-файл
  window.addEventListener('error', () => {
    getCurrentWindow().show();
  });

  // Если упадет необработанный промис
  window.addEventListener('unhandledrejection', () => {
    getCurrentWindow().show();
  });
}
