import React, { useState, useEffect, useCallback, useRef } from "react";
import { cn } from "@/lib/utils"; // Стандартная утилита shadcn для классов (clsx + tailwind-merge)

// Интерфейс пропсов
export interface ShortcutInputProps {
  value: string[]; // Массив клавиш, например ['Ctrl', 'Shift', 'K']
  onChange: (value: string[]) => void;
  placeholder?: string;
  className?: string;
}

// Утилита для форматирования названий клавиш
const formatKey = (key: string) => {
  if (key === " ") return "Space";
  if (key === "Control") return "Ctrl";
  if (key === "Meta") return "Cmd";
  if (key.length === 1) return key.toUpperCase();
  return key;
};

// Список клавиш-модификаторов
const MODIFIERS = ["Control", "Shift", "Alt", "Meta"];

export const ShortcutInput: React.FC<ShortcutInputProps> = ({
  value,
  onChange,
  placeholder = "Press to write...",
  className,
}) => {
  const [isRecording, setIsRecording] = useState(false);
  const [liveKeys, setLiveKeys] = useState<string[]>([]);
  const containerRef = useRef<HTMLDivElement>(null);

  // Обработчик нажатия клавиш
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!isRecording) return;

      e.preventDefault(); // Блокируем стандартные действия браузера
      e.stopPropagation();

      // Отмена записи по Escape
      if (e.key === "Escape") {
        setIsRecording(false);
        setLiveKeys([]);
        containerRef.current?.blur();
        return;
      }

      // Очистка по Backspace
      if (e.key === "Backspace") {
        onChange([]);
        setIsRecording(false);
        setLiveKeys([]);
        containerRef.current?.blur();
        return;
      }

      // Собираем модификаторы
      const keys: string[] = [];
      if (e.ctrlKey) keys.push("Ctrl");
      if (e.metaKey) keys.push("Cmd");
      if (e.altKey) keys.push("Alt");
      if (e.shiftKey) keys.push("Shift");

      const isModifierOnly = MODIFIERS.includes(e.key);

      if (isModifierOnly) {
        // Если нажали только модификатор, показываем его в live-режиме
        setLiveKeys(keys);
      } else {
        // Если нажата обычная клавиша (вместе с модификаторами или без)
        keys.push(formatKey(e.key));
        onChange(keys);
        setIsRecording(false);
        setLiveKeys([]);
        containerRef.current?.blur();
      }
    },
    [isRecording, onChange]
  );

  // Обработчик отпускания клавиш (нужно для live-обновления модификаторов)
  const handleKeyUp = useCallback(
    (e: KeyboardEvent) => {
      if (!isRecording) return;
      e.preventDefault();

      const keys: string[] = [];
      if (e.ctrlKey) keys.push("Ctrl");
      if (e.metaKey) keys.push("Cmd");
      if (e.altKey) keys.push("Alt");
      if (e.shiftKey) keys.push("Shift");

      setLiveKeys(keys);
    },
    [isRecording]
  );

  // Вешаем глобальные слушатели при старте записи
  useEffect(() => {
    if (isRecording) {
      window.addEventListener("keydown", handleKeyDown, { capture: true });
      window.addEventListener("keyup", handleKeyUp, { capture: true });
    }

    return () => {
      window.removeEventListener("keydown", handleKeyDown, { capture: true });
      window.removeEventListener("keyup", handleKeyUp, { capture: true });
    };
  }, [isRecording, handleKeyDown, handleKeyUp]);

  // Какие клавиши показывать (текущие записываемые или уже сохраненные)
  const displayKeys = isRecording && liveKeys.length > 0 ? liveKeys : value;

  return (
    <div
      ref={containerRef}
      tabIndex={0}
      onClick={() => setIsRecording(true)}
      onFocus={() => setIsRecording(true)}
      onBlur={() => {
        setIsRecording(false);
        setLiveKeys([]);
      }}
      className={cn(
        "flex h-10 w-full cursor-pointer items-center rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background transition-colors",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        isRecording ? "ring-2 ring-ring ring-offset-2 border-primary" : "",
        className
      )}
    >
      {displayKeys.length > 0 ? (
        <div className="flex flex-wrap gap-1">
          {displayKeys.map((key, index) => (
            <kbd
              key={index}
              className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border border-border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100"
            >
              {key}
            </kbd>
          ))}
        </div>
      ) : (
        <span className="text-muted-foreground">
          {isRecording ? "Reading keys..." : placeholder}
        </span>
      )}
    </div>
  );
};
