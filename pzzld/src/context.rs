/*
   Appellation: context <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::Settings;
use pzzld_sdk::prelude::Gateway;
use scsys::prelude::{hasher, Contextual, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub cnf: Settings,
    pub gateway: Gateway,
}

impl Context {
    pub fn new(cnf: Settings, gateway: Gateway) -> Self {
        Self { cnf, gateway }
    }
}

impl Contextual for Context {
    type Cnf = Settings;
    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        self
    }
}

impl Hashable for Context {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.cnf).unwrap())
    }
}
