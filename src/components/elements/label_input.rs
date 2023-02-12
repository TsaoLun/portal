use dioxus::prelude::*;
#[derive(Props)]
pub struct SelfProps<'a> {
    self_type: Option<&'a str>,
    name: &'a str,
    id: &'a str,
}

#[allow(non_snake_case)]
pub fn LabelInput<'a>(cx: Scope<'a, SelfProps<'a>>) -> Element<'a> {
    let input_type = if let Some(t) = cx.props.self_type {
        t
    } else {
        "text"
    };
    let style = r#"
    label {
        width: 60px;
        line-height: 3rem;
        align-self: flex-start;
        display: inline-block;
    }
    input {
        width: 300px;
        height: 30px;
        font-size: medium;
        padding-inline: 10px;
        border-radius: 0px;
        outline-color: orange;
        border: 2px solid black;
    }
    "#;
    cx.render(rsx! {
        style {"{style}"}
        label {"{cx.props.name}"}
        input {
            r#type: format_args!{"{}", input_type},
            id: format_args!{"{}", cx.props.id},
            name: format_args!{"{}", cx.props.id}
        }
    })
}
