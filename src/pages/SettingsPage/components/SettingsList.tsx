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

import { useEffect } from "react";
import useSettings from "@/app/hooks/useSettings";
import ModelProfileGroup from "./SettingsGroups/ModelProfileGroup/ModelProfileGroup";
import GeneralGroup from "./SettingsGroups/GeneralGroup";
import ProxyGroup from "./SettingsGroups/ProxyGroup";
import { Separator } from "@/components/ui/separator";
import TranslationGroup from "./SettingsGroups/TranslationGroup";
import ShortcutsGroup from "./SettingsGroups/ShortcutsGroup";

export default function () {
  const { settings, changeSettingsProperty, restoreSettings } = useSettings();
  useEffect(() => restoreSettings(), []);

  return (
    <div className="flex flex-col gap-4 max-w-xl w-[70%] mt-2 mx-auto">

      <GeneralGroup settings={settings} changeSettingsProperty={changeSettingsProperty} />
      <Separator className="my-2" />
      {settings.translationEngine === 'llm' && <ModelProfileGroup settings={settings} changeSettingsProperty={changeSettingsProperty} />}
      <Separator className="my-2" />
      <TranslationGroup settings={settings} changeSettingsProperty={changeSettingsProperty} />
      <Separator className="my-2" />
      <ProxyGroup settings={settings} changeSettingsProperty={changeSettingsProperty} />
      <Separator className="my-2" />
      <ShortcutsGroup settings={settings} changeSettingsProperty={changeSettingsProperty} />

      <br />
    </div>
  )
}
