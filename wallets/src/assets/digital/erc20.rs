/*
    Appellation: erc20 <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Describes an interface for the ERC20 fungible token standard
pub trait IERC20 {
    fn constructor() -> Self
    where
        Self: Sized;
    fn decimals(&self) -> usize;
    fn name(&self) -> String;
    fn supply(&self) -> usize;
    fn symbol(&self) -> String;
}

/// Implements the fungible token standard
pub struct ERC20 {
    pub decimals: usize,
    pub name: String,
    pub owner: String,
    pub supply: usize,
    pub symbol: String,
}

impl IERC20 for ERC20 {
    fn constructor() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn decimals(&self) -> usize {
        self.decimals
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn supply(&self) -> usize {
        self.supply
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
