mod components;
mod router;
mod store;
use dioxus::prelude::*;
use router::router;
fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    router(cx)
}
