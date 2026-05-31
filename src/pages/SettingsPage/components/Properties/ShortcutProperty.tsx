import LabelWithHint from '../LabelWithHint';
import { ShortcutInput } from '@/components/ui/ShortcutInput';
import { useState } from 'react';

interface ShortcutPropertyProps {
  id: string;
  label: string;
  defaultValue?: string[];
  onChange?: (value: string[]) => void;
  hint?: string;
}


export default ({ id, label, defaultValue, onChange, hint }: ShortcutPropertyProps) => {
  const [val, setVal] = useState(defaultValue ?? []);

  const handleChange = (newShortcut: string[]) => {
    setVal(newShortcut);
    if (onChange)
      onChange(newShortcut);
  }

  return (
    <div className='flex w-full justify-between mx-auto items-center'>
      <LabelWithHint htmlFor={id} label={label} hint={hint} />
      <ShortcutInput className='w-fit' value={val} onChange={handleChange} />
    </div>
  )
}
