/*
    Appellation: view <module>
    Contrib: @FL03
*/
use wtw::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub struct ActorView {
    pub wasi_ctx: WasiCtx,
    pub wasi_tbl: ResourceTable,
}

impl ActorView {
    pub fn new() -> Self {
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self {
            wasi_ctx: ctx,
            wasi_tbl: ResourceTable::new(),
        }
    }

    pub fn from_ctx(ctx: WasiCtx) -> Self {
        Self {
            wasi_ctx: ctx,
            wasi_tbl: ResourceTable::new(),
        }
    }

    pub const fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }

    pub fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }

    pub const fn table(&self) -> &ResourceTable {
        &self.wasi_tbl
    }

    pub fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.wasi_tbl
    }
}

impl WasiView for ActorView {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_tbl
    }
}
