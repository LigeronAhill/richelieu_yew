use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ "Hello, world!" }</h1>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}