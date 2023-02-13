use crate::components::elements::label_input::LabelInput;
use dioxus::{
    events::MouseEvent,
    prelude::{dioxus_elements::stop, *},
};
use gloo::console::*;
use web_sys::{Document, HtmlElement, HtmlInputElement, SubmitEvent};
#[allow(non_snake_case)]
pub fn Data(cx: Scope) -> Element {
    let state = use_state(&cx, || "请输入");
    let copy_style = r#"
        
    "#;
    cx.render(rsx! {
        form {
            oninput: |_| state.set("输入中..."),
            onchange: |_| state.set("请记得提交~"),
            onsubmit: |_| state.set("感谢您的提交 ^ ^"),
            onclick: |_| {log!("123");},
            prevent_default: "onsubmit",
            h1 {"{state}"}
            LabelInput{
                name:"INPUT", id:"data"
            }
            form {
                onclick: |e|{e.cancel_bubble();},
                prevent_default: "onclick",
                style {"copy_style"}
                button {
                    "C"
                }
            }
            br {}
            button {
                "提交"
            }
        }
    })
}
