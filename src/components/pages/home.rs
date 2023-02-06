use crate::{components::elements::text_input::*, store::BoardStore};
use std::ops::Deref;
use gloo::console::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::{prelude::*, dispatch};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Form />
    }
}

#[function_component(Form)]
pub fn form() -> Html {
    let (store, dispatch) = use_store::<BoardStore>();
    let onsubmit = dispatch.reduce_mut_callback_with(|state, event: SubmitEvent| {
        event.prevent_default();
        state.data = event.target_unchecked_into::<HtmlInputElement>().value();
        log!("x")
    });
    let handle_onchange = Callback::from(move |text: String| {
        log!(text);
        text
    });
    html! {
        <form onsubmit={onsubmit}>
            <h1>{&store.data}</h1>
            <TextInput name="copy" handle_onchange={handle_onchange}/>
            <div>
                <button>{"Copy"}</button>
            </div>
        </form>
    }
}
