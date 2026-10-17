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

import { Label } from "@radix-ui/react-label"
import Tooltip from "@/components/ui/Tooltip"
import { CircleAlert } from "lucide-react"
import { type LabelProps } from "@radix-ui/react-label"

type LabelWithHintProps = {
  label: string;
  hint?: string;
} & LabelProps;

export default function ({ label, hint, ...props }: LabelWithHintProps) {
  return (
    <Label className='flex items-center gap-1' {...props}>
      {label}:
      {hint && <Tooltip hint={hint} viewportMargin={40}>
        <CircleAlert size="1em" className="text-muted-foreground hover:text-foreground" />
      </Tooltip>}
    </Label>
  )
}
