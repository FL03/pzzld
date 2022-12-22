/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use pzzld::core::fnl_remove;
use scsys::prelude::{Message, Stateful, Timestamp};
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

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub message: Message,
    pub state: States,
    pub timestamp: i64,
}

impl State {
    pub fn new(message: Message, state: States) -> Self {
        let timestamp = Timestamp::default().into();
        Self {
            message,
            state,
            timestamp,
        }
    }
    pub fn state(&self) -> &States {
        &self.state
    }
    pub fn update_message(&mut self, data: serde_json::Value) -> &Self {
        self.message.push(data);
        self.update_timestamp()
    }
    pub fn update_state(&mut self, state: States) -> &Self {
        self.state = state;
        self.update_timestamp()
    }
    fn update_timestamp(&mut self) -> &Self {
        self.timestamp = Timestamp::default().into();
        self
    }
}

impl Stateful for State {
    type Data = serde_json::Value;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(Message::default(), States::default())
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}

impl From<Message> for State {
    fn from(data: Message) -> Self {
        Self::new(data, Default::default())
    }
}

impl From<States> for State {
    fn from(data: States) -> Self {
        Self::new(Default::default(), data)
    }
}

impl From<State> for Message {
    fn from(data: State) -> Self {
        data.message
    }
}

impl From<State> for States {
    fn from(data: State) -> Self {
        data.state
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let a = State::default();
        let b = States::try_from("idle").ok().unwrap();
        assert_eq!(a.state(), State::from(b).state());
    }
}
