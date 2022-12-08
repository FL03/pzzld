/*
    Appellation: digital <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use erc1155::*;
pub use erc20::*;
pub use erc721::*;

mod erc1155;
mod erc20;
mod erc721;

/// Groups together the outlined token standards into an enum
pub enum TokenStandard {
    Composite(Box<dyn IERC1155>),
    Fungible(Box<dyn IERC20>),
    NonFungible(Box<dyn IERC721>),
}

impl Default for TokenStandard {
    fn default() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        let actual = f(2, 2);
        let expected = 4;
        assert_eq!(actual, expected)
    }
}
