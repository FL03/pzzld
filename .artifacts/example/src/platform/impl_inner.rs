/*
    Appellation: context <module>
    Contrib: @FL03
*/
use super::*;
use std::sync::Arc;
use wasmtime_wasi::WasiCtx;

impl PlatformInner {
    pub fn new() -> Self {
        let config = PlatformConfig::default();
        let state = PlatformState::new();

        Self {
            config: Arc::new(config),
            state: Arc::new(state),
        }
    }

    pub fn from_config(config: PlatformConfig) -> Self {
        let state = PlatformState::new();

        Self {
            config: Arc::new(config),
            state: Arc::new(state),
        }
    }

    pub fn config(&self) -> &PlatformConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut PlatformConfig {
        Arc::make_mut(&mut self.config)
    }

    pub fn wasi(&self) -> &WasiCtx {
        self.state.wasi_ctx()
    }
}

impl Clone for PlatformInner {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            state: Arc::clone(&self.state),
        }
    }
}

impl core::ops::Deref for PlatformInner {
    type Target = PlatformConfig;

    fn deref(&self) -> &Self::Target {
        self.config()
    }
}

impl core::ops::DerefMut for PlatformInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.config_mut()
    }
}
