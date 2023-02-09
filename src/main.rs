mod app;
mod components;
mod router;
mod store;
use app::app;
fn main() {
    dioxus::web::launch(app);
}
