use dioxus::{prelude::*, events::FormEvent};
use gloo::console::*;

use crate::components::elements::label_input::LabelInput;

#[allow(non_snake_case)]
pub fn Login(cx: Scope) -> Element {
    let router = use_router(&cx);
    let onsubmit = move |evt: FormEvent| {
        let router = router.clone();
        cx.spawn(async move {
            let resp = reqwest::Client::new()
                .post("http://localhost:8080/login")
                .form(&[
                    ("username", &evt.values["username"]),
                    ("password", &evt.values["password"]),
                ])
                .send()
                .await;

            match resp {
                // Parse data from here, such as storing a response token
                Ok(_data) => log!("Login successful!"),

                //Handle any errors from the fetch here
                Err(_err) => {
                    log!("Login failed - you need a login server running on localhost:8080.");
                    router.push_route("/data", None, None);
                    
                }
            }
        });
    };
    cx.render(rsx!{
        style { [include_str!("../../assets/login.css")] }
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
