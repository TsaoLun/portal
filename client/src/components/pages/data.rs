use crate::{
    api::{data, request},
    components::elements::label_input::LabelInput,
    handlers::data_updater::copy_data,
    utils::str_tools::cut_to_show,
};
use dioxus::{
    events::{FormEvent, MouseEvent},
    prelude::*,
};
use dioxus_router::use_router;
use gloo::{console::*, dialogs::alert};

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
            if let Some(show_data) = cut_to_show(copied_data) {
                state.set(show_data);
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
