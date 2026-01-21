/*
    Appellation: worker <module>
    Contrib: @FL03
*/
use anyhow::Context;
use wasmtime::component::*;
use wasmtime::{Engine, Store};
use wasmtime_wasi::WasiView;

pub struct Worker<T> {
    linker: Linker<T>,
    store: Store<T>,
}

impl<T> Worker<T> {
    pub fn new(engine: &Engine, view: T) -> Self
    where
        T: WasiView,
    {
        // Create a new linker with the provided engine
        let mut linker = Linker::new(engine);
        // Add the command world (aka WASI CLI) to the linker
        wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();

        Self {
            linker,
            store: Store::new(engine, view),
        }
    }

    pub fn from_config(config: wasmtime::Config) -> anyhow::Result<Self>
    where
        T: Default + WasiView,
    {
        let engine = Engine::new(&config)
            .context("Failed to create the Engine with the provided configuration.")?;
        Ok(Self::new(&engine, T::default()))
    }

    pub const fn linker(&self) -> &Linker<T> {
        &self.linker
    }

    pub const fn store(&self) -> &Store<T> {
        &self.store
    }
}

impl Worker<super::view::ActorView> {}

impl Worker<crate::platform::PlatformState> {
    pub async fn add(&mut self, path: std::path::PathBuf, x: i32, y: i32) -> wasmtime::Result<i32> {
        let component =
            Component::from_file(&self.store.engine(), path).context("Component file not found")?;
        let instance =
            crate::Wasiclap::instantiate_async(&mut self.store, &component, &self.linker)
                .await
                .context("Failed to instantiate the example world")?;
        instance
            .call_add(&mut self.store, x, y)
            .await
            .context("Failed to call add function")
    }
}
