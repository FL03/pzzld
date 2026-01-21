/*
    Appellation: config <platform>
    Contrib: @FL03
*/
use crate::platform::PlatformConfig;
use anyhow::Context;
use std::path::PathBuf;

impl PlatformConfig {
    pub fn new() -> Self {
        Self::from_workdir(crate::default_workdir())
    }

    pub fn from_workdir<T>(workdir: T) -> Self
    where
        PathBuf: From<T>,
    {
        Self {
            components: Vec::default(),
            workdir: PathBuf::from(workdir),
            wasmtime: crate::wasmtime_config(),
        }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut [String] {
        &mut self.components
    }

    pub const fn wasmtime(&self) -> &wasmtime::Config {
        &self.wasmtime
    }

    pub const fn workdir(&self) -> &PathBuf {
        &self.workdir
    }

    pub fn workdir_mut(&mut self) -> &mut PathBuf {
        &mut self.workdir
    }

    pub fn set_current_dir(&self) -> anyhow::Result<()> {
        std::env::set_current_dir(&self.workdir).context("Failed to set current directory: {}")
    }

    pub fn set_workdir<T>(&mut self, workdir: T)
    where
        PathBuf: From<T>,
    {
        self.workdir = PathBuf::from(workdir);
    }

    pub fn set_workdir_option<T>(&mut self, workdir: Option<T>)
    where
        PathBuf: From<T>,
    {
        workdir.map(|w| self.set_workdir(w));
    }

    pub fn with_components<I, T>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        Self {
            components: iter.into_iter().map(|i| i.to_string()).collect(),
            ..self
        }
    }

    pub fn with_workdir(self, workdir: &str) -> Self {
        Self {
            workdir: PathBuf::from(workdir),
            ..self
        }
    }
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self::new()
    }
}
