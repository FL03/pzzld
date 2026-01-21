/*
    Appellation: fmt <module>
    Created At: 2026.01.11:07:53:18
    Contrib: @FL03
*/
#![allow(unused_macros)]

macro_rules! fmt_json {
    (@impl<json> $T:ty) => {
        impl ::core::fmt::Debug for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
            }
        }

        impl ::core::fmt::Display for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string(self).unwrap().as_str())
            }
        }
    };
    (($($T:ty),* $(,)?)) => {
        $(fmt_json! { @impl<json> $T })*
    };
}

macro_rules! debug {
    (@impl $T:ty) => {
        impl ::core::fmt::Debug for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
            }
        }
    };
    (json: $($T:ty),* $(,)?) => {
        $(debug!(@impl $T);)*
    };
}

macro_rules! display {
    (@impl $T:ty) => {
        impl ::core::fmt::Display for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string(self).unwrap().as_str())
            }
        }
    };
    (json: $($T:ty),* $(,)?) => {
        $(display!(@impl $T);)*
    };
}
