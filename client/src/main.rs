mod api;
mod components;
mod models;
mod pages;
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
