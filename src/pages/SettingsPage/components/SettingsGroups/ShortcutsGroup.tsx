import GroupHeading from './GroupHeading';
import GroupWrapper from './GroupWrapper';
import type { SettingsGroupProps } from "../../types/SettingsGroupProps";
import ShortcutProperty from '../Properties/ShortcutProperty';

export default ({ settings, changeSettingsProperty }: SettingsGroupProps) => {
  return (
    <GroupWrapper>
      <GroupHeading>Shortcuts</GroupHeading>

      <ShortcutProperty id='clear-shortcut-input' label='Clear input field shortcut' defaultValue={settings.clearShortcut}
        onChange={value => changeSettingsProperty('clearShortcut', value)} />
      <ShortcutProperty id='swap-langs-shortcut-input' label='Swap languages shortcut' defaultValue={settings.swapLangsShortcut}
        onChange={value => changeSettingsProperty('swapLangsShortcut', value)} />
      <ShortcutProperty id='apply-correction-shortcut-input' label='Apply correction shortcut' defaultValue={settings.applyCorrectionShortcut}
        onChange={value => changeSettingsProperty('applyCorrectionShortcut', value)} />

    </GroupWrapper>
  )
}
