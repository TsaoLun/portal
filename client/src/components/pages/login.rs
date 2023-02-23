use crate::{components::elements::label_input::LabelInput, handlers::data_auth::submit_login};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_router::use_router;

#[allow(non_snake_case)]
pub fn Login(cx: Scope) -> Element {
    let router = use_router(&cx);
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
        style { include_str!("../../assets/login.css") }
        div {
            h1 { "Login" }
            form {
                onsubmit: onsubmit,
                prevent_default: "onsubmit", // Prevent the default behavior of <form> to post
                LabelInput{name: "账号", id: "username"}
                br {}
                LabelInput{self_type: "password", name: "密码", id: "password"}
                br {}
                button { "登陆" }
            }
        }
    })
}
