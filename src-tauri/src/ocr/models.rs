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

//! Downloading and removing the ONNX model files.
//!
//! Files are downloaded into the app data directory, written to a temporary
//! `*.download` file first and only renamed after their exact expected size
//! has been verified, so a partially downloaded model is never treated as
//! installed.

use crate::ocr::error::OcrError;
use crate::ocr::languages::{RecModelSpec, DET_MODEL, REC_MODELS};
use reqwest::Client;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// One file that has to be present on disk for a model to work.
pub struct ModelFile {
    pub url: String,
    pub path: PathBuf,
    pub size: u64,
}

impl ModelFile {
    /// File name, used in progress events and error messages.
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

/// Files of the shared text detection model.
pub fn detection_files(models_dir: &Path) -> Vec<ModelFile> {
    vec![ModelFile {
        url: DET_MODEL.url.to_string(),
        path: models_dir.join(DET_MODEL.file),
        size: DET_MODEL.size,
    }]
}

/// Files (recognition model and dictionary) of a language model group.
pub fn recognition_files(models_dir: &Path, spec: &RecModelSpec) -> Vec<ModelFile> {
    vec![
        ModelFile {
            url: spec.rec_url(),
            path: models_dir.join(spec.rec_file),
            size: spec.rec_size,
        },
        ModelFile {
            url: spec.dict_url(),
            path: models_dir.join(spec.dict_file),
            size: spec.dict_size,
        },
    ]
}

/// Whether every file of the model exists with its expected size.
pub fn is_installed(files: &[ModelFile]) -> bool {
    files.iter().all(|file| {
        file.path
            .metadata()
            .map(|metadata| metadata.len() == file.size)
            .unwrap_or(false)
    })
}

/// Whether the shared detection model is installed.
pub fn detection_installed(models_dir: &Path) -> bool {
    is_installed(&detection_files(models_dir))
}

/// Ids of the recognition model groups that are fully installed.
pub fn installed_recognition_ids(models_dir: &Path) -> Vec<String> {
    REC_MODELS
        .iter()
        .filter(|spec| is_installed(&recognition_files(models_dir, spec)))
        .map(|spec| spec.id.to_string())
        .collect()
}

/// Creates the models directory if it is missing.
pub async fn ensure_directory(models_dir: &Path) -> Result<(), OcrError> {
    tokio::fs::create_dir_all(models_dir).await?;
    Ok(())
}

/// Downloads every missing file of a model, reporting the number of bytes
/// downloaded so far for each file.
pub async fn download_files(
    client: &Client,
    files: &[ModelFile],
    mut on_progress: impl FnMut(&ModelFile, u64),
) -> Result<(), OcrError> {
    for file in files {
        download_file(client, file, |downloaded| on_progress(file, downloaded)).await?;
    }
    Ok(())
}

/// Downloads one file unless it is already there with the expected size.
pub async fn download_file(
    client: &Client,
    file: &ModelFile,
    mut on_progress: impl FnMut(u64),
) -> Result<(), OcrError> {
    match tokio::fs::metadata(&file.path).await {
        Ok(metadata) if metadata.len() == file.size => return Ok(()),
        Ok(_) => {
            tokio::fs::remove_file(&file.path).await?;
        }
        Err(_) => {}
    }

    let temporary = temporary_path(&file.path);
    if let Err(error) = stream_to_file(client, file, &temporary, &mut on_progress).await {
        let _ = tokio::fs::remove_file(&temporary).await;
        return Err(error);
    }
    tokio::fs::rename(&temporary, &file.path).await?;
    Ok(())
}

/// Deletes the files of a model; already missing files are ignored.
pub async fn remove_files(files: &[ModelFile]) -> Result<(), OcrError> {
    for file in files {
        match tokio::fs::remove_file(&file.path).await {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

fn temporary_path(path: &Path) -> PathBuf {
    path.with_extension("download")
}

async fn stream_to_file(
    client: &Client,
    file: &ModelFile,
    destination: &Path,
    on_progress: &mut impl FnMut(u64),
) -> Result<(), OcrError> {
    let download_error = |reason: String| OcrError::Download {
        url: file.url.clone(),
        reason,
    };

    let mut response = client
        .get(&file.url)
        .send()
        .await
        .map_err(|error| download_error(error.to_string()))?;
    if !response.status().is_success() {
        return Err(download_error(format!("HTTP {}", response.status())));
    }

    let mut output = tokio::fs::File::create(destination).await?;
    let mut downloaded = 0u64;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| download_error(error.to_string()))?
    {
        output.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        on_progress(downloaded);
    }
    output.flush().await?;
    drop(output);

    if downloaded != file.size {
        return Err(OcrError::DownloadSizeMismatch {
            file: file.file_name(),
            expected: file.size,
            actual: downloaded,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installed_models_are_detected_by_size() {
        let directory = std::env::temp_dir().join("lucid-spell-ocr-models-test");
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).unwrap();

        let spec = &REC_MODELS[0];
        let files = recognition_files(&directory, spec);
        assert!(!is_installed(&files));
        assert!(installed_recognition_ids(&directory).is_empty());

        // A file of the wrong size does not count as installed.
        std::fs::write(&files[0].path, b"wrong").unwrap();
        assert!(!is_installed(&files));
    }
}
