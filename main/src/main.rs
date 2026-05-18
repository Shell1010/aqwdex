mod app;
pub mod macros;

use app::init::App;


fn main() {
    yew::Renderer::<App>::new().render();
}