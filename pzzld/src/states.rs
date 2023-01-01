/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{fnl_remove, StatePack};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Error = 0,
    #[default]
    Idle = 1,
    Request = 2,
    Response = 3,
}

impl StatePack for States {}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::prelude::{State, Stateful};

    #[test]
    fn test_state() {
        let a = State::<States>::default();
        let b = States::try_from("idle").ok().unwrap();
        assert_eq!(a.state(), b);
    }
}
