/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Mode] enumerates the possible runtime modes of the application.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::ValueEnum,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    #[clap(name = "debug")]
    #[serde(alias = "d", alias = "dev", alias = "development")]
    Debug,
    #[clap(name = "release")]
    #[serde(alias = "r", alias = "prod", alias = "production")]
    Release,
}

impl Mode {
    /// a functional constructor for the [`Debug`](Mode::Debug) variant
    pub const fn debug() -> Self {
        Self::Debug
    }
    /// a functional constructor for the [`Release`](Mode::Release) variant
    pub const fn release() -> Self {
        Self::Release
    }
}
