import { useContext } from 'react';
import { DictionariesContext, type DictionariesViewContextValue } from '../contexts/DictionariesViewContext';

export const useDictionariesView = (): DictionariesViewContextValue => {
  const context = useContext(DictionariesContext);
  if (!context) {
    throw new Error('useDictionariesView must be used within a DictionariesViewProvider');
  }
  return context;
};
