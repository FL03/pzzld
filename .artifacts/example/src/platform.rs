/*
    Appellation: platform <module>
    Contrib: @FL03
*/

mod impl_config;
mod impl_inner;
mod impl_platform;
mod impl_state;

use std::{path::PathBuf, sync::Arc};

#[derive(Default)]
pub struct Platform {
    inner: Arc<PlatformInner>,
}

#[derive(Default)]
pub struct PlatformInner {
    pub config: Arc<PlatformConfig>,
    pub state: Arc<PlatformState>,
}

/// [PlatformConfig] describes the configuration of the platform.
///
/// ## Fields
///
/// - `components`: A list of components to be loaded by the platform.
/// - `workdir`: The working directory of the platform.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, default)]
#[repr(C)]
pub struct PlatformConfig {
    pub components: Vec<String>,
    #[serde(default = "crate::default_workdir")]
    pub workdir: PathBuf,
    #[serde(skip)]
    pub wasmtime: wt::Config,
}

pub struct PlatformState {
    wasi_ctx: wtw::WasiCtx,
    wasi_res: wtw::ResourceTable,
}
