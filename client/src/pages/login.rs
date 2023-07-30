use crate::{components::label_input::LabelInput, models::data_auth::submit_login};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_router::use_router;

#[allow(non_snake_case)]
pub fn Login(cx: Scope) -> Element {
    let router = use_router(cx);
    let onsubmit = move |evt: FormEvent| {
        submit_login(
            cx,
            (
                evt.values["username"].clone(),
                evt.values["password"].clone(),
            ),
            router.clone(),
        );
    };
    cx.render(rsx! {
        div {
            class: "flex h-screen",
            div {
                class: "text-center m-auto pb-20",
                h1 {
                    class: "text-2xl ml-10", 
                    "Login" 
                }
                div {
                    class: "mt-5",
                    form { onsubmit: onsubmit, prevent_default: "onsubmit",
                        LabelInput { name: "账号", id: "username" }
                        br {}
                        LabelInput { self_type: "password", name: "密码", id: "password" }
                        div {
                            class: "ml-10 mt-5",
                            button {
                                class:"border-2 border-black hover:border-orange-500 text-center w-20 h-10 text-xl",
                                "登陆" 
                            }
                        }
                    }
                }
            }
        }
    })
}
