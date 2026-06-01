import { useContext } from 'react';
import { pages, type PageKey, type PageValue } from '@/app/consts/pages';
import { PageContext } from '@/app/contexts/PageContext';
import { cn } from '@/lib/utils';


export default function ({ className }: { className?: string }) {
  const { currentPage, changePage } = useContext(PageContext);
  return (
    <nav className={cn(
      "z-10 flex flex-row w-fit gap-2 px-2 py-0.5 border rounded-md transition-colors duration-200",
      "bg-background shadow-xs hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50", className)}
    >
      {(Object.entries(pages) as [PageKey, PageValue][]).map(([pageKey, value]) => {
        const isActive = currentPage === pageKey;

        return (
          <value.Icon key={pageKey}
            className={cn('cursor-pointer rounded-md transition-colors duration-200',
              isActive
                ? "text-foreground dark:text-foreground"
                : "text-muted-foreground hover:text-foreground "
            )}
            size={24}
            onClick={() => { if (currentPage !== pageKey) changePage(pageKey) }} />
        )
      }
      )}
    </nav>
  )
}
