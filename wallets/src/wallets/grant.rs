/*
    Appellation: grant <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::Rng;
use scsys::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AccessGrant {
    pub grant: String,
    pub timestamp: Timestamp,
}

impl AccessGrant {
    fn constructor(grant: String, timestamp: Timestamp) -> Self {
        Self { grant, timestamp }
    }
    pub fn generator(size: usize) -> String {
        let source = crate::BIP0039::default();
        let mut cache = Vec::<String>::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let random_index = rng.gen_range(0..source.0.clone().len());
            cache.push(source.0[random_index].clone())
        }

        cache.join(" ")
    }
    pub fn new(grant: String) -> Self {
        Self::constructor(grant, Timestamp::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_grant() {
        let actual = AccessGrant::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
