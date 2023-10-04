use yew::prelude::*;
use crate::counter::Counter;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="w-screen h-screen">
            <Counter value={5} />
        </div>
    }
}
