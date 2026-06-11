import { useDictionariesView } from "../hooks/useDictionariesView"
import EditorView from "./EditorView/EditorView";
import ListView from "./ListView/ListView";


export default () => {
  const { currentView } = useDictionariesView();

  let ViewComponent = ListView;
  if (currentView === 'editor') ViewComponent = EditorView

  return < ViewComponent />
}
