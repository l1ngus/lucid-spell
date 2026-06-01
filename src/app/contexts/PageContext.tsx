import { createContext, useState, type PropsWithChildren } from "react";
import { type PageKey } from '@/app/consts/pages';

export interface PageContextValue {
  currentPage: PageKey;
  changePage: (page: PageKey) => void
}

const defaultPage: PageKey = 'translator';

export const PageContext = createContext<PageContextValue>({
  currentPage: defaultPage,
  changePage: () => undefined
});

export const PageProvider = ({ children }: PropsWithChildren) => {
  const [currentPage, setCurrentPage] = useState<PageKey>(defaultPage);
  // const changePage = (page: Page) => setCurrentPage(page);
  return (
    <PageContext.Provider value={{ currentPage, changePage: setCurrentPage }} >
      {children}
    </PageContext.Provider>
  )
}
