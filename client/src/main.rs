mod api;
mod components;
mod handlers;
mod router;
mod utils;
use dioxus::prelude::*;
use router::router;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    router(cx)
}
