

use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(test)]
#[wasm_bindgen_test]
fn lib_compiles() {
    let f = |i: usize, j: usize| i + j;
    let a = f(2, 2);
    assert_eq!(a, 4);
}

#[cfg(test)]
#[wasm_bindgen_test]
async fn test_fetch() {
    let _url = "https://google.com";

    assert!(true);
}
