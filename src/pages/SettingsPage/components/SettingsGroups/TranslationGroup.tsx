import GroupHeading from './GroupHeading';
import GroupWrapper from './GroupWrapper';
import type { SettingsGroupProps } from "../../types/SettingsGroupProps";
import SwitchProperty from '../Properties/SwitchProperty';
import SliderProperty from "../Properties/SliderProperty";

export default ({ settings, changeSettingsProperty }: SettingsGroupProps) => {
  return (
    <GroupWrapper>
      <GroupHeading>Translation</GroupHeading>

      <SwitchProperty id='auto-alt-trans-fetching-switch' label='Auto alternative translations fetching'
        defaultValue={settings.isAutoAltTransFetchEnabled}
        onChange={(value) => changeSettingsProperty('isAutoAltTransFetchEnabled', value)}
        hint='If it is disabled you need to fetch them via button.' />
      <SwitchProperty id='auto-language-switching-switch' label='Auto language switching'
        defaultValue={settings.isAutoLanguageSwitchEnabled}
        onChange={(value) => changeSettingsProperty('isAutoLanguageSwitchEnabled', value)}
        hint="Language swapping after any input" />
      <SwitchProperty id='auto-translate-switch' label='Auto translate'
        defaultValue={settings.isAutoTranslateEnabled}
        onChange={(value) => changeSettingsProperty('isAutoTranslateEnabled', value)}
        hint="Translation after a small delay without pressing a button." />
      <SliderProperty label="Auto translate delay (ms)" max={2000} step={10}
        defaultValue={settings.autoTranslateDelay}
        onChange={(value) => changeSettingsProperty('autoTranslateDelay', value)}
        hint="Too low value can be dangerous because it could spam provider, which can limit your API." />
    </GroupWrapper>
  )
}
