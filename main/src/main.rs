mod app;
pub mod macros;

use app::app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}