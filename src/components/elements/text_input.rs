use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onchange: Callback<Event>,
    pub oninput: Option<Callback<InputEvent>>,
}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = &props.onchange;
    let stylesheet = style!(
        r#"
            margin-bottom: 10px;
            margin-top: 10px;
            input {
                width: 300px;
                height: 30px;
                font-size: medium;
                padding-inline: 10px;
                border-radius: 0px;
                outline-color: orange;
                border: 2px solid black;
            }
        "#
    )
    .unwrap();
    let input = {
        if let Some(oninput) = &props.oninput {
            html! {
                    <input type="text" name={props.name.clone()} onchange={onchange} oninput={oninput} placeholder={props.name.clone()} autocomplete="off"/>
            }
        } else {
            html! {
                    <input type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} autocomplete="off"/>
            }
        }
    };
    html! {<div class={stylesheet}>{input}</div>}
}
