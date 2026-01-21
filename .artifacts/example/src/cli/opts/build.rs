/*
    Appellation: build <module>
    Contrib: @FL03
*/
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, clap::Parser)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct BuildCmd {
    #[clap(long, short)]
    pub name: Option<String>,
    #[clap(long, short, default_value = "workdir")]
    #[serde(default = "crate::default_workdir")]
    pub workdir: PathBuf,
}

impl BuildCmd {
    pub async fn handle(self, ctx: &mut crate::PlatformInner) -> anyhow::Result<()> {
        ctx.set_workdir_option(Some(self.workdir));
        Ok(())
    }
}

impl Default for BuildCmd {
    fn default() -> Self {
        Self {
            name: None,
            workdir: crate::default_workdir(),
        }
    }
}
