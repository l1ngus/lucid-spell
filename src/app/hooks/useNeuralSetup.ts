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
import { commands } from "@/bindings";
import useSettings from "./useSettings";
import { AI_SERVICES } from "../consts/aiServices";

export default () => {
  const { settings, apiKeysVersion } = useSettings();

  useEffect(() => {
    const profile = settings.llmProfiles.find(p => p.id === settings.activeLlmProfileId);
    if (!profile) {
      console.error("Couldn't find selected llm profile.");
      return;
    }
    const apiUrl = profile.aiService === 'openaimanual'
      ? profile.serviceUrl
      : AI_SERVICES[profile.aiService].url;
    commands.setLlmConfig(profile.id, apiUrl, profile.model, profile.temperature)
      .then(result => {
        if (result.status === 'error')
          console.error(result.error);
      });
  }, [
    settings.activeLlmProfileId, settings.llmProfiles, apiKeysVersion
  ])
}
