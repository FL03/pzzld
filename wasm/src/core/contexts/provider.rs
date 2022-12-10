/*
    Appellation: provider <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{auth::*, errors::Error, get_token, meta::UserInfo, set_token};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    user_ctx.set(user_info.user.clone());
                }

                if let Some(error) = &current_user.error {
                    match error {
                        Error::Unauthorized | Error::Forbidden => set_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
