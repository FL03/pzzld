/*
    Appellation: app <binary>
    Contrib: @FL03
*/
use sample::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runner = wack::Runner::new();
    runner.add_package(
        wack::WasmPackage::new("component:worker", "assets/cmp/add/add.wasm").with_exports(["add"]),
    );
    runner.run("sample.wasm")?;

    let mut platform = sample::Platform::new();
    let cli = dbg!(Cli::parse());
    cli.handle(platform.as_mut()).await?;

    Ok(())
}

#[allow(unused)]
pub mod wack {
    use std::collections::HashMap;
    use wac_graph::{CompositionGraph, EncodeOptions, NodeId, types::Package};

    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
    )]
    pub struct WasmPackage {
        pub exports: Vec<String>,
        pub name: String,
        pub path: String,
        pub version: Option<String>,
    }

    impl WasmPackage {
        pub fn new(name: &str, path: &str) -> Self {
            Self {
                exports: Vec::new(),
                name: name.to_string(),
                path: path.to_string(),
                version: None,
            }
        }

        pub fn with_name(self, name: &str) -> Self {
            Self {
                name: name.to_string(),
                ..self
            }
        }

        pub fn with_path(self, path: &str) -> Self {
            Self {
                path: path.to_string(),
                ..self
            }
        }

        pub fn with_version(self, version: &str) -> Self {
            Self {
                version: Some(version.to_string()),
                ..self
            }
        }

        pub fn with_exports<I, T>(self, exports: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: ToString,
        {
            Self {
                exports: exports.into_iter().map(|i| i.to_string()).collect(),
                ..self
            }
        }

        pub fn set_exports<I, T>(&mut self, exports: I)
        where
            I: IntoIterator<Item = T>,
            T: ToString,
        {
            self.exports = exports.into_iter().map(|i| i.to_string()).collect();
        }

        pub fn set_version(&mut self, version: &str) {
            self.version = Some(version.to_string());
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }
    }

    pub struct Runner {
        pub graph: CompositionGraph,
        pub deps: Vec<WasmPackage>,
        pub exports: HashMap<String, NodeId>,
    }

    impl Runner {
        pub fn new() -> Self {
            Self {
                graph: CompositionGraph::new(),
                deps: Vec::new(),
                exports: HashMap::new(),
            }
        }

        pub fn add_package(&mut self, spec: WasmPackage) {
            self.deps.push(spec)
        }

        pub fn run(&mut self, path: &str) -> anyhow::Result<()> {
            let deps = self.deps.clone();
            deps.iter().for_each(|dep| {
                self.handler(dep).unwrap();
            });

            let bytes = self.graph.encode(EncodeOptions::default())?;

            std::fs::write(path, bytes)?;
            Ok(())
        }

        fn handler(&mut self, dep: &WasmPackage) -> anyhow::Result<()> {
            let pkg = Package::from_file(&dep.name, None, &dep.path, self.graph.types_mut())?;
            let package = self.graph.register_package(pkg)?;
            let instantiation = self.graph.instantiate(package);

            for export in dep.exports.iter() {
                let node = self.graph.alias_instance_export(instantiation, export)?;
                self.exports.insert(export.clone(), node);
            }

            Ok(())
        }
    }
}
