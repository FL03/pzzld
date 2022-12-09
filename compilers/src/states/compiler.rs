/*
   Appellation: compiler <state>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::convert::From;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum CompilerState {
    Idle = 0,
    Init = 1,
    Read = 2,
    Write = 3,
    Complete = 4,
    Invalid = 5,
}

impl CompilerState {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl Default for CompilerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl From<i64> for CompilerState {
    fn from(data: i64) -> Self {
        match data {
            0 => Self::Idle,
            1 => Self::Init,
            2 => Self::Read,
            3 => Self::Write,
            4 => Self::Complete,
            _ => Self::Invalid,
        }
    }
}

impl From<CompilerState> for i64 {
    fn from(data: CompilerState) -> Self {
        match data {
            CompilerState::Idle => 0,
            CompilerState::Init => 1,
            CompilerState::Read => 2,
            CompilerState::Write => 3,
            CompilerState::Complete => 4,
            CompilerState::Invalid => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_state() {
        let a: i64 = CompilerState::default().into();
        assert_eq!(a, 0i64);
        assert_eq!(
            CompilerState::try_from("idle").ok().unwrap(),
            CompilerState::from(a)
        )
    }
}
