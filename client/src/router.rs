use crate::pages::{data::*, login::*};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};

pub fn router(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", Data {} }
            Route { to: "/login", Login {} }
        }
    })
}
