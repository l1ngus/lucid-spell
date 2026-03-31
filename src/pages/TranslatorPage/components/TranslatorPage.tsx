import TranslatorBarActions from "./TranslatorBarActions";
import TranslatorContainer from './TransatorContainer';
import AdditionalInfo from "./AdditionalInfo/AdditionalInfo";
import { TranslationProvider } from "../contexts/TranslationContext";

export default function () {
  return (
    <TranslationProvider>
      <TranslatorBarActions />
      <TranslatorContainer />
      <AdditionalInfo />
    </TranslationProvider>
  )
}
