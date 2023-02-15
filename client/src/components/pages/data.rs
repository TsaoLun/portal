use crate::{components::elements::label_input::LabelInput, api::data};
use dioxus::{events::FormEvent, prelude::*};
use gloo::console::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

fn copy() -> bool {
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
    element.set_text_content(Some("xyz"));
    element.select();
    let res = document.exec_command("copy").unwrap();
    body.remove_child(&element).unwrap();
    res
}

#[allow(non_snake_case)]
pub fn Data(cx: Scope) -> Element {
    let state = use_state(&cx, || "请输入");
    cx.render(rsx! {
        form {
            oninput: |_| state.set("输入中..."),
            onchange: |_| state.set("请记得提交~"),
            onsubmit: move |e: FormEvent| {
                state.set("感谢您的提交 ^ ^");
                //e.values["data"];
                cx.spawn(async move {
                    let res = data::set(e.values["data"].clone()).await;
                    match res {
                        Ok(_data) => log!("ok"),
                        Err(_err) => {
                            log!(_err);
                        }
                    }
                });
            },
            prevent_default: "onsubmit",
            style { [include_str!("../../assets/data.css")] }
            h1 {"{state}"}
        
            LabelInput{
                name:"INPUT", id:"data"
            }
            button {
                onclick: move |e|{
                    e.cancel_bubble();
                    log!("copy data:", copy());
                },
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
