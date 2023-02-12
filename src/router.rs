use crate::components::pages::login::*;
use dioxus::prelude::*;

pub fn router(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/",
                login()
            }
            Route { to: "/settings",
                div {
                    h2 { margin_bottom: "10px", "Settings" }
                    button {
                        background: "rgb(202, 60, 60)",
                        class: "pure-button pure-button-primary",

                        "Remove all clients"
                    }
                    Link { to: "/", class: "pure-button pure-button-primary", "Go Back" }
                }
            }
            Route {
                to: "", "404 Not Found"
            }
        }
    })
}
