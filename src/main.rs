mod app;
mod counter;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
