use crate::{
    components::elements::{label_input::*, submit_button::*},
    router::Route,
    store::UserStore,
};
use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[styled_component(Login)]
pub fn login() -> Html {
    let handle_username = Callback::from(|_| {});
    let handle_password = Callback::from(|_| {});
    let (_store, dispatch) = use_store::<UserStore>();
    let navigator = use_navigator().unwrap();
    let onsubmit = {
        dispatch.reduce_mut_callback_with(move |_store, _event: SubmitEvent| {
            navigator.push(&Route::Data)
        })
    };
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        margin-right: 30px;
        height: 95%;
        button {
            margin-left: 60px;
        }
        h1 {
            text-align: center;
            margin-left: 60px;
            font-size: xxx-large;
        }
    "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <form onsubmit={onsubmit}>
                <h1>{"Copy That."}</h1>
                <LabelInput label_text="用户名" input_text="请输入邮箱/手机号" onchange={handle_username}/>
                <LabelInput label_text="密码" input_text="请输入 8 位密码" onchange={handle_password}/>
                <SubmitButton name="提交"/>
            </form>
        </div>
    }
}
