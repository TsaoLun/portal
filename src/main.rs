mod app;
mod components;
mod router;
mod store;
use app::app;
fn main() {
    dioxus_web::launch(app);
}
