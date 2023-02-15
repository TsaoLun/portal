mod components;
mod router;
use dioxus::prelude::*;
use router::router;
fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    router(cx)
}
