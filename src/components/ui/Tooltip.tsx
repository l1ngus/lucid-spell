/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import React, { useEffect, useRef, useState } from "react";

/**
 * Tooltip.tsx
 * Tooltip that wraps children and shows a hint on hover/focus/touch.
 * - Positions using fixed coordinates so it won't be clipped by parent overflow
 * - Multi-line hints (wrap) and expands up to the viewport width (minus viewportMargin * 2)
 * - Thin border and theme tokens `bg-background` / `text-foreground` / `border-border`
 * - Fade in/out and scale animation
 */

type Placement = "top" | "bottom" | "left" | "right";

interface TooltipProps {
  hint: React.ReactNode;
  children: React.ReactNode;
  placement?: Placement;
  delay?: number;
  className?: string;
  /** 
   * The minimum distance (in px) from the edge of the viewport.
   * Also determines the max-width (100vw - margin * 2).
   * Default: 8 
   */
  viewportMargin?: number;
}

export default function Tooltip({
  hint,
  children,
  placement = "top",
  delay = 100,
  className = "",
  viewportMargin = 10,
}: TooltipProps) {
  const [visible, setVisible] = useState(false);

  // Refs for timers
  const openTimeoutRef = useRef<number | null>(null);
  const closeTimeoutRef = useRef<number | null>(null);

  const wrapperRef = useRef<HTMLDivElement | null>(null);
  const tooltipRef = useRef<HTMLDivElement | null>(null);

  const [tooltipStyle, setTooltipStyle] = useState<React.CSSProperties>({
    position: "fixed",
    top: 0,
    left: 0,
    // Initial hidden state
    visibility: "hidden",
    opacity: 0,
    transform: "scale(0.96)",
    // Transition settings
    transition: "opacity 150ms ease-out, transform 150ms ease-out",
    maxWidth: `calc(100vw - ${viewportMargin * 2}px)`,
    zIndex: 9999,
  });

  // Cleanup timers on unmount
  useEffect(() => {
    return () => {
      if (openTimeoutRef.current) window.clearTimeout(openTimeoutRef.current);
      if (closeTimeoutRef.current) window.clearTimeout(closeTimeoutRef.current);
    };
  }, []);

  const computePosition = (currentPlacement?: Placement) => {
    if (!wrapperRef.current || !tooltipRef.current) return;

    // 1. Prepare for measurement:
    // We keep it invisible but ensure it's in the DOM to measure dimensions.
    setTooltipStyle((s) => ({
      ...s,
      visibility: "hidden",
      maxWidth: `calc(100vw - ${viewportMargin * 2}px)`
    }));

    const wRect = wrapperRef.current.getBoundingClientRect();
    // getBoundingClientRect() includes the entry animation's scale(0.96),
    // which underestimates the final size and can let a wide tooltip overflow.
    const tooltipWidth = tooltipRef.current.offsetWidth;
    const tooltipHeight = tooltipRef.current.offsetHeight;
    const offset = 8; // gap between target and tooltip (arrow space)

    let finalPlacement = currentPlacement ?? placement;
    let top = 0;
    let left = 0;

    const centerX = wRect.left + wRect.width / 2;
    const centerY = wRect.top + wRect.height / 2;

    // compute primary positions
    if (finalPlacement === "top") {
      top = wRect.top - tooltipHeight - offset;
      left = centerX - tooltipWidth / 2;
    } else if (finalPlacement === "bottom") {
      top = wRect.bottom + offset;
      left = centerX - tooltipWidth / 2;
    } else if (finalPlacement === "left") {
      top = centerY - tooltipHeight / 2;
      left = wRect.left - tooltipWidth - offset;
    } else {
      // right
      top = centerY - tooltipHeight / 2;
      left = wRect.right + offset;
    }

    // clamp horizontally and vertically using viewportMargin
    const minX = viewportMargin;
    const minY = viewportMargin;
    const maxX = window.innerWidth - tooltipWidth - viewportMargin;
    const maxY = window.innerHeight - tooltipHeight - viewportMargin;

    // Flip logic if out of bounds
    if (finalPlacement === "top" && top < minY) {
      finalPlacement = "bottom";
      top = wRect.bottom + offset;
      left = centerX - tooltipWidth / 2;
    }
    if (finalPlacement === "bottom" && top > maxY) {
      finalPlacement = "top";
      top = wRect.top - tooltipHeight - offset;
      left = centerX - tooltipWidth / 2;
    }

    // Apply clamping
    left = Math.min(Math.max(left, minX), Math.max(maxX, minX));
    top = Math.min(Math.max(top, minY), Math.max(maxY, minY));

    // 2. Set final position and trigger entry animation
    setTooltipStyle((s) => ({
      ...s,
      position: "fixed",
      top: Math.round(top),
      left: Math.round(left),
      visibility: "visible",
      zIndex: 9999,
      maxWidth: `calc(100vw - ${viewportMargin * 2}px)`,
      // Animation end state:
      opacity: 1,
      transform: "scale(1)",
    }));
  };

  const show = (immediate = false) => {
    if (openTimeoutRef.current) window.clearTimeout(openTimeoutRef.current);
    if (closeTimeoutRef.current) window.clearTimeout(closeTimeoutRef.current);

    if (immediate || delay === 0) {
      setVisible(true);
      requestAnimationFrame(() => computePosition());
    } else {
      openTimeoutRef.current = window.setTimeout(() => {
        setVisible(true);
        requestAnimationFrame(() => computePosition());
      }, delay);
    }
  };

  const hide = () => {
    if (openTimeoutRef.current) window.clearTimeout(openTimeoutRef.current);
    if (closeTimeoutRef.current) window.clearTimeout(closeTimeoutRef.current);

    setVisible(false);

    setTooltipStyle((s) => ({
      ...s,
      opacity: 0,
      transform: "scale(0.96)"
    }));

    closeTimeoutRef.current = window.setTimeout(() => {
      setTooltipStyle((s) => ({ ...s, visibility: "hidden" }));
    }, 150);
  };

  // recompute on window resize/scroll while visible
  useEffect(() => {
    if (!visible) return;
    const handler = () => computePosition();
    window.addEventListener("resize", handler);
    window.addEventListener("scroll", handler, true);

    // Trigger update if viewportMargin prop changes while visible
    handler();

    return () => {
      window.removeEventListener("resize", handler);
      window.removeEventListener("scroll", handler, true);
    };
  }, [visible, viewportMargin]);

  return (
    <div
      ref={wrapperRef}
      className={`relative inline-block ${className}`}
      onMouseEnter={() => show()}
      onMouseLeave={() => hide()}
      onFocusCapture={() => show()}
      onBlurCapture={() => hide()}
      onTouchStart={() => show(true)}
    >
      {children}

      <div
        ref={tooltipRef}
        role="tooltip"
        aria-hidden={!visible}
        style={tooltipStyle}
        className={`pointer-events-none bg-background text-foreground text-sm rounded-md px-3 py-1 shadow-xl whitespace-normal border border-border wrap-break-words`}
      >
        {hint}
      </div>
    </div>
  );
}
