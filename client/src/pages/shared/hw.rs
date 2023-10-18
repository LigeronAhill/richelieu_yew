use gloo::console::log;
use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
struct MyObject {
    name: String,
    favorite_language: String,
}

#[function_component]
pub fn HelloWorld() -> Html {
    let name = "Brooks";
    let my_object = MyObject {
        name: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("my name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class = "my_titles";
    let message: Option<&str> = Some("I am a massage");

    let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];

    html! {
        <>
            <h1 class={class}>{"Hello, world"}</h1>
            if class == "my_title" {
                <p>{"Hi there"}</p>
            } else {
                <p>{"I'm not a title"}</p>
            }
            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"no massages to see today"}</p>
            }

            <ul class="item-list">
                {list_to_html(tasks)}
            </ul>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Html {
    html! {
        <ul>
            {list.iter().map(|item| {
                html! {
                    <li>{item}</li>
                }
            }).collect::<Html>()}
        </ul>
    }
}
