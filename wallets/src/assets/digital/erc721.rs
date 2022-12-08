/*
    Appellation: erc721 <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Implements the ERC721 interface
pub trait IERC721 {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
