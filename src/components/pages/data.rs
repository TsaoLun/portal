use crate::components::elements::label_input::LabelInput;
use dioxus::prelude::*;
use gloo::console::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

#[allow(non_snake_case)]
pub fn Data(cx: Scope) -> Element {
    let state = use_state(&cx, || "请输入");
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap();
    cx.render(rsx! {
        form {
            oninput: |_| state.set("输入中..."),
            onchange: |_| state.set("请记得提交~"),
            onsubmit: |_| state.set("感谢您的提交 ^ ^"),
            prevent_default: "onsubmit",
            style { [include_str!("../../assets/data.css")] }
            h1 {"{state}"}
            form {
                onclick: move |e|{
                    e.cancel_bubble();

                    let res = document.exec_command("copy").is_ok();
                    log!(res);
                },
                prevent_default: "onclick",
                LabelInput{
                    name:"INPUT", id:"data"
                }
                button {
                    class:"copy",
                    "C"
                }
            }
            br {}
            button {
                class:"submit",
                "提交"
            }
        }
    })
}
