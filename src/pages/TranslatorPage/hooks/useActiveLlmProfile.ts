import { useMemo } from 'react';
import useSettings from '@/app/hooks/useSettings';



export default () => {
  const { settings } = useSettings();
  const llmProfile = useMemo(() => {
    const profile = settings.llmProfiles.find(p => p.id === settings.activeLlmProfileId);
    if (profile)
      return profile;
    throw new Error("Couldn't find selected llm profile.");
  }, [settings.activeLlmProfileId, settings.llmProfiles]);

  return llmProfile;
}
