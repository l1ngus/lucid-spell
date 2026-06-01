import { useState, useLayoutEffect, useEffect, type PropsWithChildren } from 'react';
import { createPortal } from 'react-dom';
import { APP_BAR_ACTIONS_ID } from './consts';

interface AppBarActionsProps extends React.HTMLAttributes<HTMLDivElement> {
}

export default function ({ children, ...props }: PropsWithChildren<AppBarActionsProps>) {
  const [container, setContainer] = useState<null | HTMLElement>(() =>
    document.getElementById(APP_BAR_ACTIONS_ID)
  );

  useLayoutEffect(() => {
    if (!container) {
      const el = document.getElementById(APP_BAR_ACTIONS_ID);
      if (el) setContainer(el);
    }
  }, [container]);

  useEffect(() => {
    if (!container) return;
    if (props.className)
      container.classList.add(...props.className.split(' '));
    if (props.style)
      Object.assign(container.style, props.style);
    return () => {
      if (props.className)
        container.classList.remove(...props.className.split(' '));
    };
  }, [container, props.className, props.style]);

  if (!container) return null;

  return createPortal(children, container);
}
