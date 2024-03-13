/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
#[test]
fn compiles() {
    let f = |i: usize| i * i;
    assert_eq!(f(2), 4);
}

#[cfg(all(target_abi = "wasm32-unknown-unknown", target_family = "wasm"))]
mod wasm {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn wasm_compiles() {
        let f = |i: usize| i * i;
        assert_eq!(f(2), 4);
    }
}
