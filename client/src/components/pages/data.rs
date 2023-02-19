use crate::{
    api::{data, request},
    components::elements::label_input::LabelInput,
};
use dioxus::{
    events::{FormEvent, MouseEvent},
    prelude::*,
};
use dioxus_router::use_router;
use gloo::{console::*, dialogs::alert};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

#[allow(non_snake_case)]
pub fn Data(cx: Scope) -> Element {
    let state = use_state(&cx, || "请输入");
    let router = use_router(&cx);
    let onclick = move |e: MouseEvent| {
        e.stop_propagation();
        cx.spawn(async move {
            let data = data::get_query(request()).await.unwrap();
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
            element.set_text_content(Some(&data));
            element.select();
            document.exec_command("copy").unwrap();
            body.remove_child(&element).unwrap();
        });
    };
    cx.render(rsx! {
        form {
            oninput: |_| state.set("输入中..."),
            onchange: |_| state.set("请记得提交~"),
            onsubmit: move |e: FormEvent| {
                let state = state.clone();
                let router = router.clone();
                cx.spawn(async move {
                    let res = data::set_mutation(request(), e.values["data"].clone()).await;
                    match res {
                        Ok(_data) => {
                            log!("data submitted!");
                            state.set("数据提交成功");
                            alert("请在任意终端按 C 复制");
                        }
                        Err(err) => {
                            log!(err.to_string());
                            router.push_route("/", None, None);
                        }
                    }
                });
            },
            prevent_default: "onsubmit",
            //style { [include_str!("../../assets/data.css")] }
            h1 {"{state}"}

            LabelInput{
                name:"INPUT", id:"data"
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
