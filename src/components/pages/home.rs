use crate::{components::elements::text_input::*, store::BoardStore};
use gloo::console::*;
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Form />
    }
}

#[function_component(Form)]
pub fn form() -> Html {
    let state = use_state(|| "not submited.");
    let (store, dispatch) = use_store::<BoardStore>();
    let onsubmit = {
        let state = state.clone();
        dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
            event.prevent_default();
            store.submit_data = store.inner_data.clone();
            state.set("submited!");
        })
    };
    let handle_onchange = dispatch.reduce_mut_callback_with(|store, event: Event| {
        store.inner_data = event.target_unchecked_into::<HtmlInputElement>().value();
    });
    html! {
        <ContextProvider<String> context={state.deref().clone()}>
            <form onsubmit={onsubmit}>
                <h1>{"STATE: "}{state.deref()}</h1>
                <TextInput name="submit" onchange={handle_onchange}/>
                <div>
                    <button>{"Submit"}</button>
                </div>
            </form>
            <h1>{&store.submit_data}</h1>
        </ContextProvider<String>>
    }
}
