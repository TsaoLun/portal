use dioxus::{events::FormEvent, prelude::*};

pub fn app(cx: Scope) -> Element {
    let onsubmit = move |evt: FormEvent| {
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
                Ok(_data) => println!("Login successful!"),

                //Handle any errors from the fetch here
                Err(_err) => {
                    println!("Login failed - you need a login server running on localhost:8080.")
                }
            }
        });
    };

    cx.render(rsx! {
        h1 { "Login" }
        form {
            onsubmit: onsubmit,
            prevent_default: "onsubmit", // Prevent the default behavior of <form> to post
            label { "Username" }
            input { r#type: "text", id: "username", name: "username" }
            br {}
            label { "Password" }
            input { r#type: "password", id: "password", name: "password" }
            br {}
            button { "Login" }
        }
    })
}
