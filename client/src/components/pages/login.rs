use crate::{
    api::{login, request},
    components::elements::label_input::LabelInput,
};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_router::use_router;
use gloo::dialogs::alert;

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
                        router.push_route("/", None, None);
                    } else {
                        alert("账号密码有误");
                    }
                }
                //Handle any errors from the fetch here
                Err(_err) => {
                    alert("服务器异常");
                }
            }
        });
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
