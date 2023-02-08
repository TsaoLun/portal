use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[styled_component(CopyButton)]
pub fn copy_button(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
            margin-top: 12px;
            margin-bottom: 12px;
            text-align: center;
            button {
                width: 40px;
                height: 40px;
                font-size: small;
                border-radius: 0px;
            }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <button>{&props.name}</button>
        </div>
    }
}
