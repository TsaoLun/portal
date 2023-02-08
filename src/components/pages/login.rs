use yew::prelude::*;
use stylist::{yew::styled_component, style};
use crate::components::elements::label_input::*;

#[styled_component(Login)]
pub fn login() -> Html {
    let handle_username = Callback::from(|_|{});
    let handle_password = Callback::from(|_|{});
    let stylesheet = style!(r#"
        display: grid;
        place-items: center;
        height: 900px;
    "#).unwrap();
    html! {
        <div class={stylesheet}>
            <div>
            <LabelInput label_text="用户名" input_text="请输入邮箱/手机号" onchange={handle_username}/>
            <LabelInput label_text="密码" input_text="请输入 8 位密码" onchange={handle_password}/>
            </div>
        </div>
    }
}
