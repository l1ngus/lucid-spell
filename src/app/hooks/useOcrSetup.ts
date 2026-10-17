/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { useEffect } from 'react';
import { commands } from '@/bindings';

export default function useOcrSetup() {
  useEffect(() => {
    let isMounted = true;

    const initializeInstalledModels = async () => {
      const statusResult = await commands.getOcrStatus();
      if (!isMounted) return;

      if (statusResult.status === 'error') {
        console.warn('Could not check OCR models:', statusResult.error);
        return;
      }

      const status = statusResult.data;
      const hasInstalledRecognitionModel = status.models.some(model => model.installed);
      if (status.initialized || !status.detectionInstalled || !hasInstalledRecognitionModel) return;

      const initializeResult = await commands.initializeOcr();
      if (!isMounted) return;

      if (initializeResult.status === 'error') {
        console.error('Could not initialize installed OCR models:', initializeResult.error);
      }
    };

    void initializeInstalledModels().catch(error => {
      if (isMounted) console.error('Could not initialize OCR:', error);
    });

    return () => {
      isMounted = false;
    };
  }, []);
}
