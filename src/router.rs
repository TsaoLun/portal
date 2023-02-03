use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::{home::*, login::*};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Login => html! {<Login />}
    }
}