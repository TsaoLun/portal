use crate::{
    components::elements::submit_button::*,
    components::elements::{copy_button::CopyButton, text_input::*},
    store::BoardStore,
};
use std::ops::Deref;
use gloo::{console::log, utils::{document_element, document}};
use stylist::{style, yew::styled_component};
use web_sys::{HtmlInputElement, Docu, HtmlElement};
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

#[function_component(Data)]
pub fn data() -> Html {
    html! {
        <Form />
    }
}

#[styled_component(Form)]
pub fn form() -> Html {
    let state = use_state(|| "请输入");
    let (_store, dispatch) = use_store::<BoardStore>();
    let onsubmit = {
        let state = state.clone();
        dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
            event.prevent_default();
            store.submit_data = store.inner_data.clone();
            state.set("感谢您的提交 ^ ^");
        })
    };
    //let d = HtmlDocument
    let handle_onclick = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
        let x = document();
        let y = x.create_element_with_str("textarea", "123").unwrap();
        //create
    });

    let handle_onchange = {
        let state = state.clone();
        dispatch.reduce_mut_callback_with(move |store, event: Event| {
            store.inner_data = event.target_unchecked_into::<HtmlInputElement>().value();
            state.set("请记得提交~");
        })
    };
    let handle_input = {
        let state = state.clone();
        dispatch.reduce_mut_callback_with(move |_store, _event: InputEvent| {
            state.set("输入中...");
        })
    };
    let stylesheet = style!(
        r#"
        text-align: center;
        margin-top: 300px;
    "#
    )
    .unwrap();
    let copysheet = style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        button {
            margin-left: 10px;
        }
    "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <ContextProvider<String> context={state.deref().clone()}>
                <form onsubmit={onsubmit}>
                    <h1>{state.deref()}</h1>
                    <div class={copysheet}>
                        <TextInput name="submit" onchange={handle_onchange} oninput={handle_input}/>
                        <span onclick={handle_onclick}>
                            <CopyButton name="C" />
                        </span>
                    </div>
                    <SubmitButton name="Submit" />
                </form>
                <h1>{&_store.submit_data}</h1>
            </ContextProvider<String>>
        </div>
    }
}
