import { useContext } from 'react';
import { pages } from '@/app/consts/pages';
import { PageContext } from '@/app/contexts/PageContext';

export default function () {
  const { currentPage } = useContext(PageContext);
  const PageComponent = pages[currentPage].Component;
  return <PageComponent />
}
