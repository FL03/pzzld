/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{Message, StatePack};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum State {
    Connect { name: String, endpoint: String },
    Idle,
}

impl State {
    pub fn into_message(&self) -> Message<Self> {
        self.clone().into()
    }
    pub fn boxed(&self) -> Box<&Self> {
        Box::new(self)
    }
    pub fn shared(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }
}

impl StatePack for State {
    
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
