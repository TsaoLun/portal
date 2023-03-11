use dioxus::prelude::*;
#[derive(Props)]
pub struct SelfProps<'a> {
    self_type: Option<&'a str>,
    name: &'a str,
    id: &'a str,
}

#[allow(non_snake_case)]
pub fn LabelInput<'a>(cx: Scope<'a, SelfProps<'a>>) -> Element {
    let input_type = if let Some(t) = cx.props.self_type {
        t
    } else {
        "text"
    };
    cx.render(rsx! {
        label {
            class: "w-10 leading-3 self-start inline-block",
            "{cx.props.name}"
        }
        input {
            class:"w-15 mt-3 ml-3 h-10 border-2 p-3 border-solid border-black",
            r#type: format_args! { "{}", input_type },
            id: format_args! { "{}", cx.props.id },
            name: format_args! { "{}", cx.props.id } }
    })
}
