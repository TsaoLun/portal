use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = input {
            handle_onchange.emit(input.value());
        }
    });
    html! {
        <input type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()}/>
    }
}
