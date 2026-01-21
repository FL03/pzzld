/*
    Appellation: consts <module>
    Created At: 2026.01.11:07:20:34
    Contrib: @FL03
*/

/// defines an array representing the localhost IP address
pub const LOCALHOST: [u8; 4] = [0, 0, 0, 0];

pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_BASEPATH: &str = "/";
pub const DEFAULT_WORKDIR: &str = ".";

pub const DEFAULT_CONFIG_FILE: &str = "Puzzled.toml";

pub const DEFAULT_DIR_ARTIFACTS: &str = ".artifacts";
pub const DEFAULT_DIR_CONFIG: &str = ".config";

pub const DEFAULT_INSTALL_DIR: &str = ".pzzld";

pub const APP_NAME: &str = "pzzld";
pub const APP_DESCRIPTION: &str = "A web framework for building scalable applications.";
/// default database connection URL
pub const DEFAULT_DB_URL: &str = "postgresql://postgres:password@localhost:5432/postgres";
