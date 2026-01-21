/*
    Appellation: actors <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::{view::ActorView, worker::Worker};

pub mod manage;
pub mod view;
pub mod worker;

use wasmtime::component::Instance;

pub trait ActorBase<T> {
    fn linker(&self) -> &wt::Linker<T>;

    fn store(&self) -> &wt::Store<T>;
}

pub trait Plugin<T> {
    type View: wtw::WasiView;

    fn invoke(&self, args: T) -> anyhow::Result<()>;
}

pub trait Manager {
    type View: wtw::WasiView;

    fn engine(&self) -> &wt::Engine;

    fn linker(&self) -> &wt::Linker<Self::View>;

    fn store(&self) -> &wt::Store<Self::View>;

    fn load_plugin(&mut self, wasm_bytes: &[u8]) -> anyhow::Result<Instance>;

    fn invoke_plugin_name(&self, name: impl ToString) -> anyhow::Result<()>;
}
