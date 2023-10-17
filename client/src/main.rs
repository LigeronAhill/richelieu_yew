use yew::prelude::*;

mod app;
mod pages;


#[function_component]
pub fn App() -> Html {

    html! {
        <h1>{"Hello, world"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
