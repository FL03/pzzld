/*
    Appellation: pzzld-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{backend::*, primitives::*, specs::*, utils::*};

pub mod clients;
pub mod events;
pub mod proxies;
pub mod servers;
pub mod signals;

pub(crate) mod backend;
pub(crate) mod primitives;
pub(crate) mod utils;

pub(crate) mod specs {
    use std::sync::Arc;

    pub trait BaseObject {
        fn count(&self) -> usize;
        fn name(&self) -> String;
        fn slug(&self) -> String {
            self.name().to_ascii_lowercase()
        }
        fn symbol(&self) -> String;
    }

    pub trait Stateful: Clone {
        fn boxed(self: Box<Self>) -> Box<Self> {
            self
        }
        fn state(self) -> Self
        where
            Self: Sized,
        {
            self
        }
        fn threaded(self: Arc<Self>) -> Arc<Self> {
            self
        }
    }

    pub trait StatefulExt: Stateful + Default {
        fn constructor() -> Self
        where
            Self: Sized;
    }

    pub trait Versionable {
        type Error;

        fn update(&mut self) -> Result<(), Box<Self::Error>>;
        fn version(&self) -> String;
    }

    pub trait BaseApplication: BaseObject + Versionable {
        fn application(&self) -> &Self {
            self
        }
        fn namespace(&self) -> String;
    }
}
