/*
    Appellation: tracing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::LogLevel;
use tracing_subscriber::filter::EnvFilter;

fn default_true() -> bool {
    true
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct TracingConfig {
    #[serde(default = "default_true")]
    pub ansi: bool,
    pub file: bool,
    pub level: LogLevel,
    pub line_number: bool,
    pub target: bool,
    pub thread_ids: bool,
    pub thread_names: bool,
    #[serde(default = "default_true")]
    pub timer: bool,
}

impl TracingConfig {
    /// initialize a new instance of the tracing configuration from the given log level
    pub const fn new(level: LogLevel) -> Self {
        Self {
            ansi: true,
            file: false,
            level,
            line_number: false,
            target: false,
            thread_ids: false,
            thread_names: false,
            timer: true,
        }
    }
    /// returns true if ansi output is enabled
    pub const fn ansi(&self) -> bool {
        self.ansi
    }
    /// returns true if file output is enabled
    pub fn file(&self) -> bool {
        self.file
    }
    /// returns a copy of the configured log level
    pub const fn level(&self) -> LogLevel {
        self.level
    }
    /// returns true if the line numbers are enabled
    pub const fn line_number(&self) -> bool {
        self.line_number
    }
    /// returns true if the target is enabled
    pub const fn target(&self) -> bool {
        self.target
    }
    /// returns true if thread ids are enabled
    pub const fn thread_ids(&self) -> bool {
        self.thread_ids
    }
    /// returns true if thread names are enabled
    pub const fn thread_names(&self) -> bool {
        self.thread_names
    }
    /// update the ansi setting and return a mutable reference to self
    pub const fn set_ansi(&mut self, ansi: bool) -> &mut Self {
        self.ansi = ansi;
        self
    }
    /// set whether to log to file or not
    pub const fn set_file(&mut self, file: bool) -> &mut Self {
        self.file = file;
        self
    }
    /// update the log level and return a mutable reference to self
    pub const fn set_level(&mut self, level: LogLevel) -> &mut Self {
        self.level = level;
        self
    }
    /// update if the line number should be displayed
    pub const fn set_line_number(&mut self, line_number: bool) -> &mut Self {
        self.line_number = line_number;
        self
    }
    /// update the target flag and return a mutable reference to self.
    pub const fn set_target(&mut self, target: bool) -> &mut Self {
        self.target = target;
        self
    }
    /// update the thread ids flag and return a mutable reference to self.
    pub const fn set_thread_ids(&mut self, thread_ids: bool) -> &mut Self {
        self.thread_ids = thread_ids;
        self
    }
    /// update the thread names flag and return a mutable reference to self.
    pub const fn set_thread_names(&mut self, thread_names: bool) -> &mut Self {
        self.thread_names = thread_names;
        self
    }

    pub const fn set_timer(&mut self, timer: bool) -> &mut Self {
        self.timer = timer;
        self
    }

    pub fn with_ansi(self, ansi: bool) -> Self {
        Self { ansi, ..self }
    }

    pub fn with_file(self, file: bool) -> Self {
        Self { file, ..self }
    }

    pub fn with_level(self, level: LogLevel) -> Self {
        Self { level, ..self }
    }

    pub fn with_line_number(self, line_number: bool) -> Self {
        Self {
            line_number,
            ..self
        }
    }

    pub fn with_target(self, target: bool) -> Self {
        Self { target, ..self }
    }

    pub fn with_thread_ids(self, thread_ids: bool) -> Self {
        Self { thread_ids, ..self }
    }

    pub fn with_thread_names(self, thread_names: bool) -> Self {
        Self {
            thread_names,
            ..self
        }
    }
    /// create an env filter from the current configuration
    pub fn get_env_filter(&self, name: &str) -> EnvFilter {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{name}={level},tower_http={level}", level = self.level).into()
        })
    }
    /// Initialize the tracer with the given name
    pub fn init_tracing(&self, name: &str) {
        use tracing_subscriber::util::SubscriberInitExt;

        let filter = self.get_env_filter(name);
        tracing_subscriber::fmt()
            .compact()
            .with_ansi(self.ansi())
            .with_env_filter(filter)
            .with_file(self.file)
            .with_line_number(self.line_number)
            .with_max_level(self.level.as_tracing_level())
            .with_target(self.target)
            .with_thread_ids(self.thread_ids)
            .with_thread_names(self.thread_names)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .finish()
            .init();
        tracing::debug! { "initialized tracing for {name}" }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self::new(LogLevel::Trace)
    }
}

impl core::fmt::Display for TracingConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
