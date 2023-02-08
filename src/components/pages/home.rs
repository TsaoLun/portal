use crate::{
    components::elements::submit_button::*, components::elements::text_input::*, store::BoardStore,
};
use gloo::console::*;
use std::ops::Deref;
use stylist::{style, yew::styled_component};
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

#[function_component(Home)]
pub fn home() -> Html {
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
    })};

    let handle_onchange = {
        let state = state.clone();
        dispatch.reduce_mut_callback_with(move |store, event: Event| {
            store.inner_data = event.target_unchecked_into::<HtmlInputElement>().value();
            state.set("请记得提交");
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
    html! {
        <div class={stylesheet}>
            <ContextProvider<String> context={state.deref().clone()}>
                <form onsubmit={onsubmit}>
                    <h1>{state.deref()}</h1>
                    <TextInput name="submit" onchange={handle_onchange} oninput={handle_input}/>
                    <SubmitButton name="Submit" />
                </form>
                <h1>{&_store.submit_data}</h1>
            </ContextProvider<String>>
        </div>
    }
}
