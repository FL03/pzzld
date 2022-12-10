/*
    Appellation: erc1155 <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Implements the ERC1155 interface
pub trait IERC1155 {
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
