import { useDictionariesView } from "../hooks/useDictionariesView"
import EditorView from "./EditorView/EditorView";
import FlashcardsView from "./FlashcardsView/FlashcardsView";
import ListView from "./ListView/ListView";


export default () => {
  const { currentView } = useDictionariesView();

  let ViewComponent = ListView;
  switch (currentView) {
    case 'editor':
      ViewComponent = EditorView;
      break;
    case 'flashcards':
      ViewComponent = FlashcardsView;
      break;
  }

  return < ViewComponent />
}
