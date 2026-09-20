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

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { ModelProfileSchema, type ModelProfile } from '@/app/types/ModelProfile';
import type { SettingsGroupProps } from "../../../types/SettingsGroupProps";
import GroupWrapper from '../GroupWrapper';
import GroupHeading from '../GroupHeading';
import TextProperty from "../../Properties/TextProperty";
import SelectProperty from "../../Properties/SelectProperty";
import SwitchProperty from '../../Properties/SwitchProperty';
import { DEFAULT_AI_SERVICE, AI_SERVICE_KEYS, AI_SERVICES } from '@/app/consts/aiServices';
import SliderProperty from '../../Properties/SliderProperty';
import ApiKeyInput from './ApiKeyInput';

export default ({ settings, changeSettingsProperty }: SettingsGroupProps) => {
  const [activeProfileId, setActiveProfileId] = useState(settings.activeLlmProfileId);
  const [profiles, setProfiles] = useState(settings.llmProfiles);
  const [currentProfile, setCurrentProfile] = useState<ModelProfile | null>(() =>
    settings.llmProfiles.find(p => p.id === settings.activeLlmProfileId) ?? null
  );

  const handleActiveProfileChange = (profileId: string) => {
    setActiveProfileId(profileId);
    const found = profiles.find(p => p.id === profileId) ?? null;
    setCurrentProfile(found);
    changeSettingsProperty("activeLlmProfileId", profileId);
  }

  const handleCreateNewProfile = () => {
    const newProfile: ModelProfile = {
      id: crypto.randomUUID(),
      name: 'New profile',
      aiService: DEFAULT_AI_SERVICE,
      serviceUrl: '',
      // isProxyEnabled: false,
      model: '',
      temperature: 0.5
    }
    const updated = [...profiles, newProfile];
    setProfiles(updated);
    setActiveProfileId(newProfile.id);
    setCurrentProfile(newProfile);
    changeSettingsProperty("llmProfiles", updated);
    changeSettingsProperty("activeLlmProfileId", newProfile.id);
  }

  const handleDeleteActiveProfile = () => {
    const updated = profiles.filter(p => p.id !== activeProfileId);
    setCurrentProfile(null);
    setProfiles(updated);
    changeSettingsProperty('llmProfiles', updated);
  }

  const handleProfileFieldChange = <K extends keyof ModelProfile>(field: K, value: ModelProfile[K]) => {
    if (!currentProfile) return;
    const updatedProfile = { ...currentProfile, [field]: value } as ModelProfile;
    const updatedProfiles = profiles.map(p => (p.id === updatedProfile.id ? updatedProfile : p));
    setCurrentProfile(updatedProfile);
    setProfiles(updatedProfiles);
    changeSettingsProperty("llmProfiles", updatedProfiles);
  };

  const isProfileIdValid = profiles.some(p => p.id === activeProfileId);

  return (
    <GroupWrapper>
      <GroupHeading>Model profile</GroupHeading>
      <div className="flex items-end gap-2">
        <SelectProperty label='Profile'
          value={isProfileIdValid
            ? activeProfileId
            : ""}
          selectItems={[
            ...profiles.map(p => ({
              label: p.name,
              value: p.id
            }))]}
          onChange={handleActiveProfileChange}
          placeholder='Select an LLM profile.' />
        <Button variant='outline' onClick={handleCreateNewProfile}>Add</Button>
        <Button disabled={!isProfileIdValid}
          className='border-destructive hover:bg-destructive'
          variant='outline' onClick={handleDeleteActiveProfile}
        >Del</Button>
      </div>
      {isProfileIdValid &&
        <TextProperty id='profile-name-input' label='Profile name'
          value={currentProfile?.name ?? ""}
          onChange={val => handleProfileFieldChange('name', val)} />}
      {isProfileIdValid &&
        <SelectProperty label="AI service"
          value={currentProfile?.aiService ?? DEFAULT_AI_SERVICE}
          selectItems={AI_SERVICE_KEYS.map(service => ({
            label: AI_SERVICES[service].label,
            value: service
          }))}
          onChange={val => handleProfileFieldChange('aiService', val)} />}

      {(isProfileIdValid && currentProfile?.aiService === 'openaimanual') &&
        <TextProperty id='profile-service-url-input' label='AI service URL'
          value={currentProfile?.serviceUrl ?? ""}
          onChange={val => handleProfileFieldChange('serviceUrl', val)} />}
      {/* {isProfileIdValid && */}
      {/*   <TextProperty id='profile-service-api-key-input' label='API key' */}
      {/*     value={currentProfile?.apiKey ?? ""} */}
      {/*     onChange={val => handleProfileFieldChange('apiKey', val)} */}
      {/*     type='password' />} */}
      {isProfileIdValid &&
        <ApiKeyInput currentProfile={currentProfile} />}
      {/* {isProfileIdValid && */}
      {/*   <SwitchProperty id='profile-proxy-switch' label='Use proxy for this profile' */}
      {/*     checked={currentProfile?.isProxyEnabled ?? false} */}
      {/*     onChange={value => handleProfileFieldChange('isProxyEnabled', value)} />} */}
      {isProfileIdValid &&
        <TextProperty id='profile-model-input' label='Model'
          value={currentProfile?.model ?? ""}
          onChange={val => handleProfileFieldChange('model', val)} />}
      {isProfileIdValid &&
        <SliderProperty label='Temperature' value={currentProfile?.temperature ?? ModelProfileSchema.shape.temperature.parse(undefined)}
          min={0} max={2} step={0.1} onChange={value => handleProfileFieldChange("temperature", value)}
          hint='Temperature controls how literal or flexible the AI’s translations are: 0.0 – 0.2 Strict; 0.3 – 0.5 Natural; 0.7+ Creative' />}

    </GroupWrapper>
  )
}
