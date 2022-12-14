/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use pzzld::core::fnl_remove;
use serde::{Deserialize, Serialize};
use std::convert::From;
use strum::{EnumString, EnumVariantNames};

pub trait Stateful {
    fn by_ref(self: &Self) -> Self where Self: Sized;
    fn state(&self) -> Self where Self: Sized;
}


#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Request {
        methods: serde_json::Value
    },
    Response,
    Idle,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(serde_json::to_string(&self).unwrap()).to_ascii_lowercase()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let a = State::default();
        let b = State::try_from("idle").ok().unwrap();
        assert_eq!(a, b);
        assert_eq!(a.to_string(), "idle".to_string())
    }
}
