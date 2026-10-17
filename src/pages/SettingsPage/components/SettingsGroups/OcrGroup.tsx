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

import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import {
  Check,
  CircleAlert,
  Cpu,
  Download,
  HardDrive,
  Languages,
  LoaderCircle,
  ScanText,
  Trash2,
} from 'lucide-react';
import { commands, type OcrDownloadProgress, type OcrModelInfo, type OcrStatusResponse } from '@/bindings';
import { Button } from '@/components/ui/button';
import GroupHeading from './GroupHeading';
import GroupWrapper from './GroupWrapper';

type OcrOperation = {
  modelId: string;
  action: 'install' | 'remove' | 'initialize';
} | null;

function formatBytes(bytes: number): string {
  if (bytes >= 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${bytes} bytes`;
}

function formatLanguages(languages: string[]): string {
  if (languages.length <= 5) return languages.map(language => language.toUpperCase()).join(', ');
  return `${languages.length} languages`;
}

function getOperationLabel(operation: OcrOperation, models: OcrModelInfo[]): string {
  if (!operation) return '';
  if (operation.action === 'initialize') return 'Starting OCR…';

  const modelName = operation.modelId === 'detection'
    ? 'OCR detector'
    : models.find(model => model.id === operation.modelId)?.name ?? 'recognition model';

  return operation.action === 'install'
    ? `Downloading ${modelName}…`
    : `Removing ${modelName}…`;
}

export default function OcrGroup() {
  const [status, setStatus] = useState<OcrStatusResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [operation, setOperation] = useState<OcrOperation>(null);
  const [progress, setProgress] = useState<OcrDownloadProgress | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let isMounted = true;
    let unlisten: (() => void) | undefined;

    void listen<OcrDownloadProgress>('ocr-download-progress', event => {
      if (isMounted) setProgress(event.payload);
    }).then(unlistenFn => {
      if (isMounted) {
        unlisten = unlistenFn;
      } else {
        unlistenFn();
      }
    }).catch(eventError => {
      if (isMounted) console.error('Could not listen for OCR download progress:', eventError);
    });

    return () => {
      isMounted = false;
      unlisten?.();
    };
  }, []);

  useEffect(() => {
    let isMounted = true;

    void commands.getOcrStatus().then(result => {
      if (!isMounted) return;
      if (result.status === 'error') {
        setError(result.error);
        return;
      }
      setStatus(result.data);
    }).catch(statusError => {
      if (isMounted) setError(String(statusError));
    }).finally(() => {
      if (isMounted) setIsLoading(false);
    });

    return () => {
      isMounted = false;
    };
  }, []);

  const initializeIfReady = async (nextStatus: OcrStatusResponse, modelId: string) => {
    setStatus(nextStatus);
    const hasInstalledRecognitionModel = nextStatus.models.some(model => model.installed);
    if (!nextStatus.detectionInstalled || !hasInstalledRecognitionModel) return;

    setOperation({ modelId, action: 'initialize' });
    const result = await commands.initializeOcr();
    if (result.status === 'error') {
      setError(`Models were downloaded, but OCR could not start: ${result.error}`);
      return;
    }
    setStatus(result.data);
  };

  const installModel = async (modelId: string, languages: string[]) => {
    setOperation({ modelId, action: 'install' });
    setProgress(null);
    setError(null);

    try {
      const result = await commands.installOcrLanguages({ languages });
      if (result.status === 'error') {
        setError(result.error);
        return;
      }
      await initializeIfReady(result.data, modelId);
    } catch (installError) {
      setError(installError instanceof Error ? installError.message : String(installError));
    } finally {
      setOperation(null);
      setProgress(null);
    }
  };

  const removeModel = async (modelId: string, language: string) => {
    setOperation({ modelId, action: 'remove' });
    setProgress(null);
    setError(null);

    try {
      const result = await commands.uninstallOcrLanguage(language);
      if (result.status === 'error') {
        setError(result.error);
        return;
      }
      setStatus(result.data);
    } catch (removeError) {
      setError(removeError instanceof Error ? removeError.message : String(removeError));
    } finally {
      setOperation(null);
    }
  };

  const installDetector = () => installModel('detection', []);

  const removeDetector = async () => {
    setOperation({ modelId: 'detection', action: 'remove' });
    setProgress(null);
    setError(null);

    try {
      const result = await commands.uninstallOcrDetectionModel();
      if (result.status === 'error') {
        setError(result.error);
        return;
      }
      setStatus(result.data);
    } catch (removeError) {
      setError(removeError instanceof Error ? removeError.message : String(removeError));
    } finally {
      setOperation(null);
    }
  };

  const models = status?.models ?? [];
  const isBusy = operation !== null;
  const installedModelCount = models.filter(model => model.installed).length;
  const progressModel = progress?.model === 'detection'
    ? 'OCR detector'
    : models.find(model => model.id === progress?.model)?.name ?? 'Recognition model';
  const progressPercent = Math.round((progress?.progress ?? 0) * 100);

  return (
    <GroupWrapper className="gap-3">
      <GroupHeading>OCR</GroupHeading>

      <p className="mx-auto -mt-1 max-w-lg text-center text-sm text-muted-foreground">
        Read text from images locally. Download the shared detector and the recognition models for the languages you need.
      </p>

      {error && (
        <div role="alert" className="flex items-start gap-2 rounded-lg border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive">
          <CircleAlert className="mt-0.5 size-4 shrink-0" aria-hidden="true" />
          <span>{error}</span>
        </div>
      )}

      {isBusy && (
        <div className="rounded-lg border bg-muted/30 p-3" aria-live="polite">
          <div className="flex items-center gap-2 text-sm font-medium">
            <LoaderCircle className="size-4 animate-spin text-primary" aria-hidden="true" />
            <span>{getOperationLabel(operation, models)}</span>
          </div>
          {operation?.action === 'install' && progress && (
            <div className="mt-3 space-y-1.5">
              <div className="flex items-center justify-between gap-3 text-xs text-muted-foreground">
                <span className="truncate">{progressModel} · {progress.fileName}</span>
                <span className="shrink-0">{progressPercent}%</span>
              </div>
              <div
                role="progressbar"
                aria-label={`Downloading ${progress.fileName}`}
                aria-valuemin={0}
                aria-valuemax={100}
                aria-valuenow={progressPercent}
                className="h-2 overflow-hidden rounded-full bg-secondary"
              >
                <div className="h-full rounded-full bg-primary transition-[width]" style={{ width: `${progressPercent}%` }} />
              </div>
              <p className="text-right text-xs text-muted-foreground">
                {formatBytes(progress.downloadedBytes)} / {formatBytes(progress.totalBytes)}
              </p>
            </div>
          )}
        </div>
      )}

      <section aria-labelledby="ocr-detector-heading" className="rounded-xl border bg-card p-4 shadow-sm">
        <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div className="flex min-w-0 items-start gap-3">
            <div className="rounded-lg bg-primary/10 p-2 text-primary">
              <ScanText className="size-5" aria-hidden="true" />
            </div>
            <div className="min-w-0">
              <div className="flex flex-wrap items-center gap-2">
                <h3 id="ocr-detector-heading" className="font-semibold">Text detector</h3>
                {status?.detectionInstalled && (
                  <span className="inline-flex items-center gap-1 rounded-full bg-emerald-500/10 px-2 py-0.5 text-xs font-medium text-emerald-700 dark:text-emerald-400">
                    <Check className="size-3" aria-hidden="true" /> Installed
                  </span>
                )}
                {status?.detectionLoaded && (
                  <span className="inline-flex items-center gap-1 rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">
                    <Cpu className="size-3" aria-hidden="true" /> Running
                  </span>
                )}
              </div>
              <p className="mt-1 text-sm text-muted-foreground">
                Shared by every recognition model. Removing it turns OCR off until it is downloaded again.
              </p>
              <p className="mt-2 flex items-center gap-1.5 text-xs text-muted-foreground">
                <HardDrive className="size-3.5" aria-hidden="true" />
                {status ? formatBytes(status.detectionSizeBytes) : '…'}
              </p>
            </div>
          </div>
          <Button
            type="button"
            variant={status?.detectionInstalled ? 'outline' : 'default'}
            className="shrink-0"
            disabled={isLoading || isBusy || !status}
            onClick={status?.detectionInstalled ? removeDetector : installDetector}
          >
            {status?.detectionInstalled ? <Trash2 aria-hidden="true" /> : <Download aria-hidden="true" />}
            {status?.detectionInstalled ? 'Remove' : 'Download'}
          </Button>
        </div>
      </section>

      <div className="flex items-end justify-between gap-3 pt-1">
        <div>
          <h3 className="font-semibold">Recognition models</h3>
          <p className="text-sm text-muted-foreground">Choose the scripts you want OCR to recognize.</p>
        </div>
        {!isLoading && status && (
          <span className="shrink-0 text-xs text-muted-foreground">
            {installedModelCount} of {models.length} installed
          </span>
        )}
      </div>

      {isLoading ? (
        <div className="flex items-center justify-center gap-2 rounded-xl border p-6 text-sm text-muted-foreground" role="status">
          <LoaderCircle className="size-4 animate-spin" aria-hidden="true" />
          Checking installed models…
        </div>
      ) : status ? (
        <div className="grid gap-1.5 sm:grid-cols-2">
          {models.map(model => (
            <section key={model.id} className="flex min-w-0 items-center gap-2 rounded-lg border bg-card px-2.5 py-2">
              <div className="min-w-0 flex-1">
                <div className="flex min-w-0 items-center gap-1.5">
                  <h4 className="truncate text-sm font-medium">{model.name}</h4>
                  {model.loaded && (
                    <span className="shrink-0 rounded-full bg-primary/10 px-1.5 py-0.5 text-[10px] font-medium leading-none text-primary">Running</span>
                  )}
                  {model.installed && !model.loaded && (
                    <span className="shrink-0 rounded-full bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium leading-none text-emerald-700 dark:text-emerald-400">Installed</span>
                  )}
                </div>
                <p
                  className="mt-0.5 flex min-w-0 items-center gap-1 text-xs leading-tight text-muted-foreground"
                  title={`${model.languages.map(language => language.toUpperCase()).join(', ')} · ${formatBytes(model.sizeBytes)}`}
                >
                  <Languages className="size-3 shrink-0" aria-hidden="true" />
                  <span className="truncate">{formatLanguages(model.languages)}</span>
                  <span aria-hidden="true">·</span>
                  <span className="shrink-0">{formatBytes(model.sizeBytes)}</span>
                </p>
              </div>
              <Button
                type="button"
                size="icon-sm"
                variant={model.installed ? 'outline' : 'secondary'}
                className="size-7 shrink-0"
                aria-label={`${model.installed ? 'Remove' : 'Download'} ${model.name} recognition model`}
                title={`${model.installed ? 'Remove' : 'Download'} ${model.name} model`}
                disabled={isBusy || !model.languages[0]}
                onClick={() => model.installed
                  ? void removeModel(model.id, model.languages[0])
                  : void installModel(model.id, [model.languages[0]])}
              >
                {model.installed ? <Trash2 aria-hidden="true" /> : <Download aria-hidden="true" />}
              </Button>
            </section>
          ))}
        </div>
      ) : (
        <div className="rounded-xl border p-4 text-center text-sm text-muted-foreground">
          Recognition models are unavailable. Check the error above and try again later.
        </div>
      )}

      {status?.initialized && (
        <p className="flex items-center justify-center gap-1.5 text-xs text-muted-foreground" role="status">
          <Check className="size-3.5 text-emerald-600 dark:text-emerald-400" aria-hidden="true" />
          OCR starts automatically when installed models are available.
        </p>
      )}
    </GroupWrapper>
  );
}
