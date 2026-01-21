/*
    Appellation: context <module>
    Created At: 2026.01.10:16:28:22
    Contrib: @FL03
*/
use crate::config::Settings;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Context {
    pub(crate) settings: Settings,
}

impl Context {
    /// attempts to build the settings from the configured sources, falling back to the default
    pub fn new() -> Self {
        let settings = Settings::build().unwrap_or_default();
        Self { settings }
    }
    /// initialize a new context from the given configuration
    pub const fn from_config(settings: Settings) -> Self {
        Self { settings }
    }
    /// returns a reference to the [`Settings`] of the current context.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }
    /// returns a mutable reference to the [`Settings`] of the current context.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
}
