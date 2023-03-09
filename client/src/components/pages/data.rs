use crate::{
    components::elements::label_input::LabelInput,
    handlers::data_updater::{copy_data, first_cache, submit_data},
    utils::str_tools::{cut_to_show, portal},
};
use dioxus::{
    events::{FormEvent, MouseEvent},
    prelude::*,
};
use dioxus_router::use_router;

#[allow(non_snake_case)]
pub fn Data(cx: Scope) -> Element {
    let router = use_router(cx);
    let state = use_state(cx, || "请输入，或按 C 复制".to_string());
    let (init_data, copied_data) = (
        use_state(cx, || "".to_string()),
        use_state(cx, || "".to_string()),
    );
    // init data
    use_effect(cx, (), |_| {
        to_owned![init_data, router];
        async move {
            first_cache(init_data, router).await;
        }
    });
    // watch copied data
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
    let onsubmit = move |e: FormEvent| {
        submit_data(
            cx,
            (state.clone(), e.values["data"].clone()),
            router.clone(),
        );
    };
    let ontouchstart = move |_| {
        copy_data(cx, (copied_data.clone(), init_data.clone()), router.clone());
    };
    let ontouchend = move |_| {
        portal(copied_data.clone().to_string());
    };
    cx.render(rsx! {
        form {
            oninput: |_| state.set("输入中...".into()),
            onchange: |_| state.set("请记得提交~".into()),
            onsubmit: onsubmit,
            prevent_default: "onsubmit",
            class: "text-center ml-4",
            style { include_str!("../../assets/data.css") }
            h1 {
                class: "text-2xl mb-6 mr-3", 
                "{state}" 
            }

            LabelInput { name: "", id: "data" }
            button {
                onclick: onclick,
                ontouchstart: ontouchstart,
                ontouchend: ontouchend,
                prevent_default: "onclick",
                class: "border-2 border-black w-10 h-10 ml-6",
                "C"
            }

            br {}
            button { 
                class: "border-2 border-black w-20 h-10 text-xl mt-5 mr-3", 
                "提交" 
            }
        }
    })
}
