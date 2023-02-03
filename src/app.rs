use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::prelude::*;
use crate::router::{switch, Route};

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter><Switch<Route> render={switch}/></BrowserRouter>
    }
}