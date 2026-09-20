
import { useEffect } from "react";
import { commands } from "@/bindings";
import useSettings from "./useSettings";

export default () => {
  const { settings } = useSettings();

  useEffect(() => {
    let proxyUrl = null
    if (settings.isProxyEnabled) {
      proxyUrl = `${settings.proxyProtocol}://${settings.proxyUser}:${settings.proxyPass}@${settings.proxyHost}:${settings.proxyPort}`
    }
    commands.setProxy(proxyUrl)
      .then(result => {
        if (result.status === 'error')
          console.error(result.error);
      });
  }, [
    settings.isProxyEnabled,
    settings.proxyHost, settings.proxyPort, settings.proxyProtocol,
    settings.proxyUser, settings.proxyPass
  ])
}
