use std::ops::Deref;
use yew::prelude::*;
use crate::components::elements::{text_input::*};

#[derive(Clone)]
struct HomeData {
    data: String,
    is_login: bool    
}

impl Default for HomeData {
    fn default() -> Self {
        HomeData { data: "".to_string(), is_login: false }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<String>
}

#[function_component(Home)]
pub fn home() -> Html {
    let home_data = use_state(HomeData::default);
    let portal_data = {
        Callback::from(move |data: String| {
            let v = home_data.deref().clone();
            home_data.set(HomeData { data, ..v });
        })
    };
    html! {
        <Form onsubmit={portal_data}/>
    }
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    let state = use_state(|| "");
    let form_onsubmit = props.onsubmit.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        form_onsubmit.emit(state.to_string());
    });
    let handle_onchange = Callback::from(move |text: String|{

    });
    html! {
        <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={handle_onchange}/>
        </form>
    }
}