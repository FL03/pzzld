/*
    Appellation: add <module>
    Contrib: @FL03
*/

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, clap::Parser)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct AddCmd {
    /// The first operand
    lhs: i32,
    /// The second operand
    rhs: i32,

    #[clap(long, short, default_value = "add")]
    /// the name of the componenet
    component: String,
}

impl AddCmd {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }

    pub async fn handle(self, ctx: &mut crate::PlatformInner) -> anyhow::Result<()> {
        let _wtc = ctx.config().wasmtime();
        // let alt = if ctx.is_release() {
        //     ctx.app_dir().join("release").join("add.wasm")
        // } else {
        //     ctx.app_dir().join("debug").join("add.wasm")
        // };
        // Construct the path to the component file
        let path = ctx
            .config()
            .workdir()
            .join(format!("{}.wasm", self.component));
        debug_assert!(path.exists() && path.is_file() && path.ends_with(".wasm"));
        // Call the `add` function in the component
        let sum = crate::cli::add(path, self.lhs, self.rhs).await?;

        println!("add({lhs}, {rhs}) = {sum}", lhs = self.lhs, rhs = self.rhs);
        Ok(())
    }
}
