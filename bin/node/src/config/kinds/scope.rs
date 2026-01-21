/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::path::PathBuf;

fn _default_context() -> Option<String> {
    Some(".".to_string())
}

fn _default_workdir() -> String {
    std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| ".".to_string())
}

/// [Scope] stores critical information regarding the applications current position within
/// the filesystem. The context is considered to be the current working directory of the
/// application while the workdir is used to point to the directory where all of the assets
/// are stored.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default, deny_unknown_fields, rename_all = "snake_case")]
#[repr(C)]
pub struct Scope {
    // The root directory of the service
    pub(crate) context: Option<String>,
    // The directory where all of the assets
    #[serde(default = "_default_workdir")]
    pub(crate) workdir: String,
}

impl Scope {
    pub fn from_workdir<T>(workdir: T) -> Self
    where
        T: ToString,
    {
        Self {
            context: None,
            workdir: workdir.to_string(),
        }
    }
    /// returns a reference to the context of the scope
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
    /// returns a mutable reference to the context of the scope
    pub fn context_mut(&mut self) -> Option<&mut String> {
        self.context.as_mut()
    }
    /// returns a reference to the workdir
    pub fn workdir(&self) -> &str {
        &self.workdir
    }
    /// convert the scope into a [`PathBuf`], automatically including the context if it exists
    pub fn as_path(&self) -> PathBuf {
        // initialize a new path
        let mut path = PathBuf::new();
        // include the context, if it exists
        self.context().clone().map(|context| path.push(context));
        // add the workdir
        path.push(self.workdir());
        // ensure the path is a directory
        debug_assert!(path.is_dir());
        // return the path
        path
    }
    /// converts the scope into a string
    pub fn as_path_str(&self) -> String {
        self.as_path().display().to_string()
    }
    /// consumes the current instance to ensure no context is present
    pub fn contextless(self) -> Self {
        Self {
            context: None,
            ..self
        }
    }
    /// sets the current working directory to the scope
    pub fn set_cwd(&self) {
        std::env::set_current_dir(self.as_path()).unwrap();
    }
    /// sets the context of the scope
    pub fn set_context<T>(&mut self, context: T)
    where
        T: ToString,
    {
        self.context = Some(context.to_string())
    }
    /// update the current workind directory
    pub fn set_workdir<T>(&mut self, workdir: T)
    where
        T: ToString,
    {
        self.workdir = workdir.to_string()
    }
    /// returns a string representation of the scope
    pub fn display(&self) -> String {
        self.as_path().display().to_string()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            context: None,
            workdir: ".".into(),
        }
    }
}

impl core::fmt::Display for Scope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{path}", path = self.display())
    }
}

impl core::str::FromStr for Scope {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_workdir(s))
    }
}
