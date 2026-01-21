/*
    Appellation: state <module>
    Contrib: @FL03
*/
use crate::platform::PlatformState;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

impl PlatformState {
    pub fn new() -> Self {
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self {
            wasi_ctx: ctx,
            wasi_res: ResourceTable::new(),
        }
    }

    pub const fn wasi_ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }

    pub fn wasi_ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }

    pub const fn table(&self) -> &ResourceTable {
        &self.wasi_res
    }

    pub fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.wasi_res
    }
}

impl Default for PlatformState {
    fn default() -> Self {
        Self::new()
    }
}

impl WasiView for PlatformState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_res
    }
}
