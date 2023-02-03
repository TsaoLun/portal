use gloo::console::log;
use yew::prelude::*;

#[function_component(Login)]
pub fn home() -> Html {
    log!("login");
    html! {
        <div>
            <h1>{"Login"}</h1>
        </div>
    }
}