/*
    Appellation: connect <module>
    Created At: 2026.01.10:15:53:44
    Contrib: @FL03
*/

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Parser,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct ConnectCmd {
    #[clap(subcommand)]
    pub args: Option<ConnectOpts>,
    #[clap(long, short = 'H')]
    pub host: Option<String>,
    #[clap(long, short)]
    pub port: Option<u16>,
    #[clap(long, short)]
    pub workdir: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum ConnectOpts {
    Run {
        #[clap(long, short)]
        prefix: Option<String>,
    },
}

/*
 ************* Implementations *************
*/

impl ConnectCmd {
    pub fn new() -> Self {
        clap::Parser::parse()
    }
    /// returns a reference to the optional command arguments
    pub fn args(&self) -> Option<&ConnectOpts> {
        self.args.as_ref()
    }
    /// returns a reference to the configured host
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }
    /// returns the port
    pub fn port(&self) -> Option<u16> {
        self.port
    }
    /// handle the current command with the given context
    #[tracing::instrument(skip_all, target = "cli")]
    pub async fn handle(&self, _ctx: &mut crate::Context) -> anyhow::Result<()> {
        tracing::info!("Handling 'connect' command with args: {:?}", self);
        Ok(())
    }
}
