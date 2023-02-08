use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[styled_component(SubmitButton)]
pub fn submit_button(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
            margin-top: 12px;
            margin-bottom: 12px;
            button {
                width: 100px;
                height: 50px;
                font-size: large;
            }
        "#
    ).unwrap();
    html! {
        <div class={stylesheet}>
            <button>{&props.name}</button>
        </div>
    }
}