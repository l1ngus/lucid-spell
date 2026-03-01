import { useEffect } from "react";
import { AppSettingsSchema } from "../types/AppSettings";
import useSettings from "./useSettings";


export default () => {
  const { settings } = useSettings();

  useEffect(() => {
    const theme = settings.theme;
    const html = document.documentElement;
    html.classList.remove(...AppSettingsSchema.shape.theme.unwrap().options);
    html.classList.add(theme);
  }, [settings.theme]);
}
