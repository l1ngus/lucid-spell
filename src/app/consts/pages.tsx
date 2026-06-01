import { HomeIcon, SettingsIcon } from "lucide-react";
import SettingsPage from "@/pages/SettingsPage";
import TranslatorPage from "@/pages/TranslatorPage";

export const pages = {
  settings: {
    Component: SettingsPage,
    Icon: SettingsIcon
  },
  translator: {
    Component: TranslatorPage,
    Icon: HomeIcon
  }
};

export type PageKey = keyof typeof pages;
export type PageValue = (typeof pages)[PageKey]
