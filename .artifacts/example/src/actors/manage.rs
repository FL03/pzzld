/*
    Appellation: manage <module>
    Contrib: @FL03
*/
use anyhow::Context;
use std::collections::HashMap;
use wasmtime::component::{Component, ComponentExportIndex, types::ComponentItem};
use wasmtime::{AsContextMut, Engine};
use wasmtime_wasi::WasiView;

pub trait PluginView: Default + WasiView {}

pub struct Plugin<T = Box<dyn WasiView>> {
    pub index: ComponentExportIndex,
    pub component: ComponentItem,
    pub name: String,
    pub view: T,
}

pub struct ComponentManager {
    engine: Engine,
    components: HashMap<String, Plugin>,
}

impl ComponentManager {
    pub fn new() -> Self {
        // Create a new engine
        let engine = Engine::default();

        Self {
            engine,
            components: HashMap::new(),
        }
    }

    pub const fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn load_plugin<P, T>(
        &mut self,
        name: &str,
        path: P,
        view: T,
    ) -> anyhow::Result<Option<Plugin>>
    where
        P: AsRef<std::path::Path>,
        T: WasiView + 'static,
    {
        let cmp = Component::from_file(&self.engine, path).context("Component module not found")?;

        let (item, idx) = cmp
            .export_index(None, name)
            .context("Failed to find main function")?;

        let plugin = Plugin {
            index: idx,
            component: item,
            name: name.to_string(),
            view,
        };
        // if an item with the same name already exists, replace and return the old one
        let out = self
            .components
            .insert(plugin.name().to_string(), plugin.into_dyn());

        Ok(out)
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Plugin> {
        self.components.get(name)
    }
}

impl<T> Plugin<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn invoke(&self, cx: impl AsContextMut, args: T) -> anyhow::Result<()> {
        println!("Invoking plugin with name: {}", self.name);
        // self.component.get_func(cx, &self.name, args).context("Failed to invoke plugin")?;
        Ok(())
    }

    pub fn as_dyn(&self) -> Plugin<Box<dyn WasiView>>
    where
        T: Clone + WasiView + 'static,
    {
        Plugin {
            index: self.index,
            component: self.component.clone(),
            name: self.name.clone(),
            view: Box::new(self.view.clone()),
        }
    }

    pub fn into_dyn(self) -> Plugin<Box<dyn WasiView>>
    where
        T: WasiView + 'static,
    {
        Plugin {
            index: self.index,
            component: self.component,
            name: self.name,
            view: Box::new(self.view),
        }
    }
}
