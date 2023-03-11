mod api;
mod components;
mod models;
mod router;
mod utils;
mod pages;
use dioxus::prelude::*;
use router::router;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    router(cx)
}
