mod app;
use app::app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}