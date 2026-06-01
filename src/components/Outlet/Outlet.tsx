import { useContext } from 'react';
import { pages } from '@/app/consts/pages';
import { PageContext } from '@/app/contexts/PageContext';

export default function () {
  const { currentPage } = useContext(PageContext);

  return (
    <>
      {currentPage === 'translator' && <pages.translator.Component />}
      {currentPage === 'settings' && <pages.settings.Component />}
    </>
  )
}
