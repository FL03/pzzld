/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::prelude::AsyncResult;

/// Implements an interface for standard, multi-device authenticators
pub trait IAuthenticator<Addr: std::string::ToString, Data>:
    Clone + PartialEq + std::fmt::Debug
{
    fn get(&self) -> AsyncResult<Self> {
        Ok(self.clone())
    }
    fn authenticate(&self, address: Addr, signature: String) -> AsyncResult<bool>
    where
        Self: Sized,
    {
        let mut authenticated: bool = false;
        let sig = signature.clone();
        if address.to_string() == "".to_string() {
            authenticated = true;
        }
        Ok(authenticated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticator_interface() {
        #[derive(Clone, Debug, Hash, PartialEq)]
        struct App {
            address: String,
            datastore: Vec<String>,
        }
        impl IAuthenticator<String, Vec<String>> for App {}
        impl Default for App {
            fn default() -> Self {
                Self {
                    address: String::new(),
                    datastore: Vec::<String>::new(),
                }
            }
        }
        let actual = App::default();
        let expected = actual.clone();
        assert_eq!(actual.get().ok().unwrap(), expected.get().ok().unwrap())
    }
}
