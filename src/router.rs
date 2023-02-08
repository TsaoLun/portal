use crate::components::pages::{data::*, login::*};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/data")]
    Data,
    #[at("/login")]
    Login,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Data => html! {<Data />},
        Route::Login => html! {<Login />},
    }
}
