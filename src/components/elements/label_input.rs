use super::text_input::*;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label_text: String,
    pub input_text: String,
    pub onchange: Callback<Event>,
}

#[styled_component(LabelInput)]
pub fn label_input(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
            display: flex;
            align-items: center;
            label {
                width: 60px;
                line-height: 3rem;
            }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <label>{&props.label_text}</label>
            <TextInput name={props.input_text.to_owned()} onchange={&props.onchange} />
        </div>
    }
}
