use crate::pages::shared::style_hw::MyStyledComponent;
pub fn render_hello_world() {
    yew::Renderer::<MyStyledComponent>::new().render();
}
