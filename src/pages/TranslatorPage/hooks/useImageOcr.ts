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

import { useCallback } from 'react';
import { commands } from '@/bindings';

const OCR_UNAVAILABLE_MESSAGE = 'OCR is disabled or unavailable. Install OCR models in Settings to recognize images.';

function readImageAsDataUrl(image: Blob): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result === 'string') resolve(reader.result);
      else reject(new Error('Could not read the image.'));
    };
    reader.onerror = () => reject(reader.error ?? new Error('Could not read the image.'));
    reader.readAsDataURL(image);
  });
}

export default function useImageOcr() {
  return useCallback(async (image: Blob, lang: string): Promise<string> => {
    let statusResult;
    try {
      statusResult = await commands.getOcrStatus();
    } catch {
      throw new Error(OCR_UNAVAILABLE_MESSAGE);
    }

    if (statusResult.status === 'error' || !statusResult.data.initialized || !statusResult.data.detectionLoaded) {
      throw new Error(OCR_UNAVAILABLE_MESSAGE);
    }

    const languageModel = statusResult.data.models.find(model => model.languages.includes(lang));
    if (!languageModel) {
      throw new Error(`OCR does not support ${lang}. Check the available models in Settings.`);
    }
    if (!languageModel.installed || !languageModel.loaded) {
      throw new Error(`The OCR model for ${lang} is not installed. Install it in Settings.`);
    }

    const imageBase64 = await readImageAsDataUrl(image);
    const result = await commands.ocrImage({
      imageBase64,
      imagePath: null,
      lang,
    });

    if (result.status === 'error') throw new Error(result.error);
    return result.data.text;
  }, []);
}
