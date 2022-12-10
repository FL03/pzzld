/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::{
    prelude::{Message, Stateful},
    Timestamp,
};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum State<T: Default + std::fmt::Display> {
    Request {
        msg: Message<T>
    },
    Response {
        msg: Message<T>
    },
    Idle,
}

impl<T: Default + std::fmt::Display> Default for State<T> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T: Default + Serialize + std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
