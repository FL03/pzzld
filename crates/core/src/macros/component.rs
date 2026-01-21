/*
    appellation: format <module>
    authors: @FL03
*/
#![cfg(feature = "macros")]

/// A procedural macro for creating wasm componenets
#[macro_export]
macro_rules! component {
    ($($cmp:ident);* $(;)?) => {
        $($crate::component! { @impl $cmp })*
    };
    (@impl $cmp:ident) => {

    };
}
