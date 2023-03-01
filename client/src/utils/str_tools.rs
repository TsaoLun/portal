use dioxus::prelude::UseState;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

pub fn portal(text: String) {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap();
    let element = document
        .create_element("textarea")
        .unwrap()
        .unchecked_into::<HtmlTextAreaElement>();
    let body = document.body().unwrap();
    body.append_with_node_1(&element).unwrap();
    element.set_text_content(Some(&text));
    element.select();
    document.exec_command("copy").unwrap();
    body.remove_child(&element).unwrap();
}

pub fn cut_to_show(text: UseState<String>) -> Option<String> {
    let vec: Vec<char> = text.chars().clone().collect();
    if vec.is_empty() {
        None
    } else if vec.len() < 3 {
        Some(format!(
            "复制成功: {}",
            vec[0..vec.len()].iter().collect::<String>()
        ))
    } else if vec.len() < 5 {
        Some(format!(
            "复制成功: {}{}",
            vec[0..2].iter().collect::<String>(),
            if vec.len() == 3 { "*" } else { "**" }
        ))
    } else {
        Some(format!(
            "复制成功: {}{}",
            vec[0..2].iter().collect::<String>(),
            "**..."
        ))
    }
}
