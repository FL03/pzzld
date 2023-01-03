/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{fnl_remove, Hash, Hashable, Message, StatePack};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

pub type State = scsys::prelude::State<States>;

#[derive(
    Clone, Copy, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Error = 0,
    #[default]
    Idle = 1,
    ReqRes = 2,
}

impl StatePack for States {}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}

impl From<States> for i64 {
    fn from(data: States) -> Self {
        data as i64
    }
}

impl From<i64> for States {
    fn from(data: i64) -> Self {
        match data {
            0 => Self::Error,
            1 => Self::Idle,
            2 => Self::ReqRes,
            _ => Self::Error,
        }
    }
}

impl TryInto<Value> for States {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Value, <States as TryInto<Value>>::Error> {
        let res = serde_json::to_value(State::new(None, None, Some(self)))?;
        Ok(res)
    }
}

impl TryInto<Message> for States {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Message, <States as TryInto<Message>>::Error> {
        let res: Value = self.try_into()?;
        Ok(Message::from(res))
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
