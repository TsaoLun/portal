use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onchange: Callback<Event>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = &props.onchange;
    html! {
        <input type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()}/>
    }
}
