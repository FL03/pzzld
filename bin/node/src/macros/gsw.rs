/*
    Appellation: gsw <module>
    Contrib: @FL03
*/
#![allow(unused_macros)]
/// The `gsw` macro generates getter and setter methods for the fields of a struct. At the
/// moment, the macro can handle any type; for types that implement the [`Copy`] trait, simply
/// drop the `&` to the left of each type.
macro_rules! gsw {
    ($($name:ident: &$T:ty),* $(,)?) => {
        $(
            getter! { $name: &$T }
            set_with! { $name: $T }
        )*
    };
    ($($name:ident: $T:ty),* $(,)?) => {
        $(
            getter! { $name: $T }
            set_with! { $name: $T }
        )*
    };
}
macro_rules! getter {
    ($name:ident: &$T:ty) => {
        getter! { @get $name: &$T }
        getter! { @get_mut $name: $T }
    };
    ($name:ident: $T:ty) => {
        getter! { @get $name: $T }
        getter! { @get_mut $name: $T }
    };
    (@get_mut $name:ident: $T:ty) => {
        paste::paste! {
            pub const fn [<$name _mut>] (&mut self) -> &mut $T {
                &mut self.$name
            }
        }
    };
    (@get $name:ident: &$T:ty) => {
        pub const fn $name(&self) -> &$T {
            &self.$name
        }
    };
    (@get $name:ident: $T:ty) => {
        pub const fn $name(&self) -> $T {
            self.$name
        }
    };
}

macro_rules! set_with {
    ($($name:ident: $T:ty),* $(,)?) => {
        set_with! { @set $name: $T }
        set_with! { @with $name: $T }
    };
    (@set $name:ident: $T:ty) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $T) -> &mut Self {
                self.$name = $name;
                self
            }
        }
    };
    (@with $name:ident: $T:ty) => {
        paste::paste! {
            pub fn [<with_ $name>] (self, $name: $T) -> Self {
                Self {
                    $name,
                    ..self
                }
            }
        }
    };
}
