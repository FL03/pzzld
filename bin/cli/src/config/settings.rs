/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{kinds::*, types::*};

type ConfigBuilder<S = config::builder::DefaultState> = config::builder::ConfigBuilder<S>;

type ConfigResult<T> = core::result::Result<T, config::ConfigError>;

fn _load_env_or_default(env: &str, default: impl ToString) -> String {
    std::env::var(env).unwrap_or_else(|_| default.to_string())
}

fn source_file(
    dir: impl ToString,
    name: impl ToString,
) -> config::File<config::FileSourceFile, config::FileFormat> {
    let fname = format!("{p}/{f}", p = dir.to_string(), f = name.to_string());
    config::File::with_name(&fname)
}

fn with_sources<T: core::fmt::Display>(
    ctx: ConfigBuilder,
    workdir: &str,
    names: impl IntoIterator<Item = T>,
) -> ConfigBuilder {
    let mut tmp = ctx;
    for n in names {
        tmp = tmp.add_source(source_file(workdir, n).required(false));
    }
    tmp
}

fn set_default(builder: ConfigBuilder) -> ConfigResult<ConfigBuilder> {
    let builder = builder
        .set_default("mode", "debug")?
        .set_default("name", crate::config::APP_NAME)?
        .set_default("version", env!("CARGO_PKG_VERSION"))?
        .set_default("scope.context", ".")?
        .set_default("scope.workdir", crate::config::DEFAULT_WORKDIR)?
        .set_default("network.address.host", crate::config::DEFAULT_HOST)?
        .set_default("network.address.port", crate::config::DEFAULT_PORT)?
        .set_default("services.tracing.level", "info")?;
    Ok(builder)
}

fn add_sources(builder: ConfigBuilder) -> ConfigBuilder {
    let workdir = _load_env_or_default("APP_CONFIG_DIR", crate::config::DEFAULT_DIR_CONFIG);
    // get the settings file name
    let fname = _load_env_or_default("APP_CONFIG_FILE", crate::config::DEFAULT_CONFIG_FILE);
    // setup the builder's sources
    let builder = with_sources(
        builder,
        &workdir,
        &[
            "default.config",
            "debug.config",
            "development.config",
            "app.config",
            "prod.config",
            &fname,
        ],
    );

    builder
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .add_source(config::File::with_name(&fname).required(false))
}

fn set_overrides(builder: ConfigBuilder) -> ConfigResult<ConfigBuilder> {
    Ok({
        builder
            .set_override_option("mode", std::env::var("APP_MODE").ok())?
            .set_override_option("name", std::env::var("APP_NAME").ok())?
            .set_override_option("network.address.host", std::env::var("APP_HOST").ok())?
            .set_override_option("network.address.port", std::env::var("APP_PORT").ok())?
            .set_override_option("workspace.workdir", std::env::var("APP_WORKDIR").ok())?
    })
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "snake_case")]
pub struct Settings {
    pub mode: Mode,
    pub name: String,
    pub network: NetworkConfig,
    pub scope: Scope,
    pub services: ServicesConfig,
    pub version: String,
    pub workspace: WorkspaceConfig,
}

impl Settings {
    pub const DEFAULT_MODE: Mode = Mode::Debug;
    pub const APP_NAME: &'static str = "arachnid";
    /// attempts to initialize a [`build`](Self::build) a new instance of the configuration,
    /// falling back to the logical default if unsuccessful
    pub fn new() -> Self {
        Self::build().unwrap_or_default()
    }
    /// initialize a new instance of the settings from the given [`Mode`]
    pub fn from_mode(mode: Mode) -> Self {
        Self {
            mode,
            name: Self::APP_NAME.to_string(),
            network: NetworkConfig::default(),
            scope: Scope::default(),
            services: ServicesConfig::default(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            workspace: WorkspaceConfig::default(),
        }
    }
    /// consumes the current instance to create another in [`Debug`](Mode::Debug) mode
    pub fn debug(self) -> Self {
        Self {
            mode: Mode::Debug,
            ..self
        }
    }
    /// consumes the current instance to create another in [`Release`](Mode::Release) mode
    pub fn release(self) -> Self {
        Self {
            mode: Mode::Release,
            ..self
        }
    }
    /// tries to build the settings from the configuration sources
    pub fn build() -> Result<Self, config::ConfigError> {
        Self::builder_base()?.build()?.try_deserialize()
    }
    /// returns a copy of the configured [`Mode`]
    pub const fn mode(&self) -> Mode {
        self.mode
    }
    /// returns a reference to the application name
    pub fn name(&self) -> &str {
        &self.name
    }
    /// returns a reference to the network configuration
    pub const fn network(&self) -> &NetworkConfig {
        &self.network
    }
    /// returns a mutable reference to the network configuration
    pub const fn network_mut(&mut self) -> &mut NetworkConfig {
        &mut self.network
    }
    /// returns a reference to the current scope
    pub const fn scope(&self) -> &Scope {
        &self.scope
    }
    /// returns a mutable reference to the current scope
    pub const fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
    }
    /// returns a reference to the services
    pub const fn services(&self) -> &ServicesConfig {
        &self.services
    }
    /// returns a mutable reference to the services
    pub const fn services_mut(&mut self) -> &mut ServicesConfig {
        &mut self.services
    }
    /// returns the version of the application
    pub fn version(&self) -> &str {
        &self.version
    }
    /// returns a reference to the workspace
    pub const fn workspace(&self) -> &WorkspaceConfig {
        &self.workspace
    }
    /// returns a mutable reference to the workspace
    pub const fn workspace_mut(&mut self) -> &mut WorkspaceConfig {
        &mut self.workspace
    }
    #[cfg(feature = "net")]
    /// attempts to bind a [`TcpListener`](tokio::net::TcpListener) to the configured network
    /// address.
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        self.network().bind().await
    }
    /// Initialize tracing modules
    pub fn init_tracing(&self) {
        self.services().tracing().init_tracing(self.name());
    }
    /// set the working directory of the scope
    pub fn set_workdir<T>(&mut self, workdir: T)
    where
        std::path::PathBuf: From<T>,
    {
        self.workspace_mut().set_workdir(workdir.into());
    }
    /// if the workdir is set, set it to the given workdir
    pub fn set_workdir_option<T>(&mut self, workdir: Option<T>)
    where
        std::path::PathBuf: From<T>,
    {
        workdir.map(|w| self.set_workdir(w));
    }

    pub fn set_port(&mut self, port: u16) {
        self.network_mut().set_port(port);
    }

    pub fn set_log_level(&mut self, level: LogLevel) {
        self.services_mut().tracing_mut().set_level(level);
    }

    fn builder_base() -> ConfigResult<ConfigBuilder> {
        // initialize the builder
        let mut builder = config::Config::builder();
        // set defaults
        builder = set_default(builder)?;
        // add sources
        builder = add_sources(builder);
        // set overrides
        builder = set_overrides(builder)?;
        // return the builder
        Ok(builder)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::from_mode(Mode::Debug)
    }
}

impl core::fmt::Debug for Settings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).unwrap())
    }
}

impl core::fmt::Display for Settings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}
