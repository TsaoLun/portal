mod app;
mod components;
mod router;
mod store;
use app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}
