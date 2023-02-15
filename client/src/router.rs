use crate::components::pages::{login::*, data::*};
use dioxus::prelude::*;

pub fn router(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", Login {} }
            Route { to: "/data", Data {} }
            Route { to: "", "404 Not Found" }
        }
    })
}
