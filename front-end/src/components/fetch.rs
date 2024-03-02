use leptos::*;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

// Define a structure that matches the JSON data format you expect from the API.
#[derive(Deserialize, Debug)]
struct Post {
    title: String,
}

// Asynchronously fetches the title of the first post from JSONPlaceholder.
async fn fetch_post_title() -> Result<String, String> {
    // Prepare the request to the API.
    let url = "https://jsonplaceholder.typicode.com/posts";
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = match Request::new_with_str_and_init(url, &opts) {
        Ok(req) => req,
        Err(_) => return Err("Failed to create request".into()),
    };

    let window = web_sys::window().expect("no global `window` exists");
    let fetch = window.fetch_with_request(&request);

    // Perform the fetch operation and process the response.
    let response = JsFuture::from(fetch)
        .await
        .map_err(|_| "Network error")
        .and_then(|resp| {
            resp.dyn_into::<Response>()
                .map_err(|_| "Failed to cast response".into())
        })?;

    if response.ok() {
        let json = JsFuture::from(response.json().unwrap())
            .await
            .map_err(|_| "Failed to parse JSON")?;

        // Deserialize the JSON into a Vec<Post> and extract the title of the first post.
        let posts: Vec<Post> = from_value(json).map_err(|_| "Failed to deserialize JSON")?;
        posts
            .get(0)
            .map(|post| post.title.clone())
            .ok_or_else(|| "Post title not found".into())
    } else {
        Err("Fetch request failed".into())
    }
}

// A Leptos component that fetches and displays a post title.
#[component]
pub fn FetchComponent() -> impl IntoView {
    let (post_title, set_post_title) = create_signal(String::from("Loading..."));

    // Fetch the post title when the component is mounted.
    let async_result = move || {
        // let set_post_title = set_post_title.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match fetch_post_title().await {
                Ok(title) => {
                    console::log_1(&"Request created successfully".into());
                    // Assuming `title` is a String
                    console::log_1(&JsValue::from_str(&title));

                    set_post_title.set(title)
                } // Correctly call the setter
                Err(error_message) => set_post_title.set(error_message), // Correctly call the setter
            }
        });
    };

    view! {
        <div>
        <p>testing</p>
            <p>{post_title.get()}</p>
            <p>data {async_result}</p>
        </div>
    }
}
