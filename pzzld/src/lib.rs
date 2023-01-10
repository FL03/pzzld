/*
    Appellation: pzzld <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{primitives::*, utils::*};

pub(crate) mod utils;

pub mod api;

use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

pub(crate) mod primitives {
    use wasm_bindgen::prelude::JsError;

    /// Type alias for a [Result] of type T and error [JsError]
    pub type JsResult<T = ()> = Result<T, JsError>;
}
