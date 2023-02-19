use crate::{
    api::{login, request},
    components::elements::label_input::LabelInput,
};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_router::use_router;
use gloo::console::*;

#[allow(non_snake_case)]
pub fn Login(cx: Scope) -> Element {
    let router = use_router(&cx);
    let onsubmit = move |evt: FormEvent| {
        let router = router.clone();
        cx.spawn(async move {
            let resp = login::login(
                request(),
                evt.values["username"].clone(),
                evt.values["password"].clone(),
            )
            .await;

            match resp {
                // Parse data from here, such as storing a response token
                Ok(ok) => {
                    if ok {
                        log!("Login successful!");
                        router.push_route("/data", None, None);
                    } else {
                        log!("login fail.");
                    }
                }
                //Handle any errors from the fetch here
                Err(_err) => {
                    log!("Login failed - you need a login server running on 127.0.0.1:8080.");
                }
            }
        });
    };
    cx.render(rsx! {
        //style { [include_str!("../../assets/login.css")] }
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
