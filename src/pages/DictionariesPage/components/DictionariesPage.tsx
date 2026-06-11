import DictionariesBarActions from "./DictionariesBarActions"
import { DictionariesViewProvider } from "../contexts/DictionariesViewContext"
import ViewOutlet from "./ViewOutlet"

export default () => {
  return (
    <DictionariesViewProvider>
      <DictionariesBarActions />
      <ViewOutlet />
    </DictionariesViewProvider>
  )
}
