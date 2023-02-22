use std::rc::Rc;

use crate::{
    api::{data, request},
    components::elements::label_input::LabelInput,
};
use dioxus::{
    events::{FormEvent, MouseEvent},
    prelude::*,
};
use dioxus_router::{use_router, RouterService};
use gloo::{console::*, dialogs::alert};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

#[allow(non_snake_case)]
pub fn Data(cx: Scope) -> Element {
    let router = use_router(cx);
    let state = use_state(cx, || "请输入，或按 C 复制".to_string());
    let copied_data = use_state(cx, || "".to_string());
    let init_data = use_state(cx, || "".to_string());
    use_effect(cx, (), |_| {
        to_owned![init_data, router];
        async move {
            let data = data::get_query(request()).await;
            match data {
                Ok(data) => {
                    init_data.set(data);
                }
                Err(_) => {
                    alert("登录过期，请重新登录");
                    router.push_route("/login", None, None);
                }
            }
        }
    }); // 提供初始化
    use_effect(cx, copied_data, |_| {
        to_owned![state, copied_data];
        async move {
            let cp: Vec<char> = copied_data.chars().clone().collect();
            let (desc, len, ends) = {
                log!(cp.len().to_string());
                if cp.len() == 0 {
                    ("", 0, "")
                } else if cp.len() <= 2 {
                    ("复制成功:", cp.len(), "")
                } else if cp.len() <= 4 {
                    ("复制成功:", 2, if cp.len() == 3 { "*" } else { "**" })
                } else {
                    ("复制成功:", 2, "**...")
                }
            };
            if len != 0 {
                let data = cp[0..len].iter().collect::<String>();
                let show_success = format!("{} {}{}", desc, data, ends);
                state.set(show_success);
            }
        }
    });
    let onclick = move |e: MouseEvent| {
        e.stop_propagation();
        copy_data(cx, (copied_data.clone(), init_data.clone()), router.clone());
    };
    cx.render(rsx! {
        form {
            oninput: |_| state.set("输入中...".into()),
            onchange: |_| state.set("请记得提交~".into()),
            onsubmit: move |e: FormEvent| {
                let state = state.clone();
                let router = router.clone();
                cx.spawn(async move {
                    let res = data::set_mutation(request(), e.values["data"].clone()).await;
                    match res {
                        Ok(_data) => {
                            log!("data submitted!");
                            state.set("在任意终端按 C 复制".into());
                        }
                        Err(err) => {
                            log!(err.to_string());
                            alert("登录过期，请重新登录");
                            router.push_route("/login", None, None);
                        }
                    }
                });
            },
            prevent_default: "onsubmit",
            style { include_str!("../../assets/data.css") }
            h1 {"{state}"}

            LabelInput{
                name:"", id:"data"
            }
            button {
                onclick: onclick,
                prevent_default: "onclick",
                class:"copy",
                "C"
            }

            br {}
            button {
            class:"submit",
                "提交"
            }

        }
    })
}

fn copy_data(
    cx: Scope,
    (copied_data, init_data): (UseState<String>, UseState<String>),
    router: Rc<RouterService>,
) {
    if copied_data.get() == "" {
        portal(init_data.get().into());
    } else {
        portal(copied_data.get().into());
    }
    cx.spawn(async move {
        update_copied_data(copied_data, router).await;
    });
}

async fn update_copied_data(copied_data: UseState<String>, router: Rc<RouterService>) {
    let data = data::get_query(request()).await;
    match data {
        Ok(data) => {
            portal(data.to_string());
            copied_data.set(data);
        }
        Err(_) => {
            alert("登录过期，请重新登录");
            router.push_route("/login", None, None);
        }
    }
}

fn portal(d: String) {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap();
    let element = document
        .create_element("textarea")
        .unwrap()
        .unchecked_into::<HtmlTextAreaElement>();
    let body = document.body().unwrap();
    body.append_with_node_1(&element).unwrap();
    element.set_text_content(Some(&d));
    element.select();
    document.exec_command("copy").unwrap();
    body.remove_child(&element).unwrap();
}
