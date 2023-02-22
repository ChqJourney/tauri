// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::Result;
use crate::{plugin::PluginHandle, Runtime};
use std::path::PathBuf;

/// A helper class to access the mobile camera APIs.
pub struct PathResolver<R: Runtime>(pub(crate) PluginHandle<R>);

#[derive(serde::Deserialize)]
struct PathResponse {
  path: PathBuf,
}

#[cfg(target_os = "android")]
impl<R: Runtime> PathResolver<R> {
  fn resolve(&self, dir: &str) -> Result<PathBuf> {
    self
      .0
      .run_mobile_plugin::<PathResponse>(dir, ())
      .map(|r| r.path)
      .map_err(Into::into)
  }

  /// Returns the path to the user's audio directory.
  pub fn audio_dir(&self) -> Result<PathBuf> {
    self.resolve("getAudioDir")
  }

  /// Returns the path to the user's cache directory.
  pub fn cache_dir(&self) -> Result<PathBuf> {
    self.resolve("getExternalCacheDir")
  }

  /// Returns the path to the user's config directory.
  pub fn config_dir(&self) -> Result<PathBuf> {
    self.resolve("getConfigDir")
  }

  /// Returns the path to the user's data directory.
  pub fn data_dir(&self) -> Result<PathBuf> {
    self.resolve("getDataDir")
  }

  /// Returns the path to the user's local data directory.
  pub fn local_data_dir(&self) -> Result<PathBuf> {
    self.resolve("getDataDir")
  }

  /// Returns the path to the user's document directory.
  pub fn document_dir(&self) -> Result<PathBuf> {
    self.resolve("getDocumentDir")
  }

  /// Returns the path to the user's download directory.
  pub fn download_dir(&self) -> Result<PathBuf> {
    self.resolve("getDownloadDir")
  }

  /// Returns the path to the user's picture directory.
  pub fn picture_dir(&self) -> Result<PathBuf> {
    self.resolve("getPictureDir")
  }

  /// Returns the path to the user's public directory.
  pub fn public_dir(&self) -> Result<PathBuf> {
    self.resolve("getPublicDir")
  }

  /// Returns the path to the user's video dir
  pub fn video_dir(&self) -> Result<PathBuf> {
    self.resolve("getVideoDir")
  }

  /// Returns the path to the resource directory of this app.
  pub fn resource_dir(&self) -> Result<PathBuf> {
    self.resolve("getResourcesDir")
  }

  /// Returns the path to the suggested directory for your app's config files.
  ///
  /// Resolves to [`config_dir`]`/${bundle_identifier}`.
  pub fn app_config_dir(&self) -> Result<PathBuf> {
    self.resolve("getConfigDir")
  }

  /// Returns the path to the suggested directory for your app's data files.
  ///
  /// Resolves to [`data_dir`]`/${bundle_identifier}`.
  pub fn app_data_dir(&self) -> Result<PathBuf> {
    self.resolve("getDataDir")
  }

  /// Returns the path to the suggested directory for your app's local data files.
  ///
  /// Resolves to [`local_data_dir`]`/${bundle_identifier}`.
  pub fn app_local_data_dir(&self) -> Result<PathBuf> {
    self.resolve("getDataDir")
  }

  /// Returns the path to the suggested directory for your app's cache files.
  ///
  /// Resolves to [`cache_dir`]`/${bundle_identifier}`.
  pub fn app_cache_dir(&self) -> Result<PathBuf> {
    self.resolve("getCacheDir")
  }

  /// Returns the path to the suggested directory for your app's log files.
  pub fn app_log_dir(&self) -> Result<PathBuf> {
    self.resolve("getConfigDir").map(|dir| dir.join("logs"))
  }
}