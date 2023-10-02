use yew::prelude::*;

pub mod counter;
use crate::counter::Counter;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Counter value={5} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
