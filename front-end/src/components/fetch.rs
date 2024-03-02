use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

async fn fetch_post() -> Result<String, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let fetch = window.fetch_with_str("https://jsonplaceholder.typicode.com/posts/1");

    let response = JsFuture::from(fetch).await?.dyn_into::<Response>()?;
    if response.ok() {
        let json = JsFuture::from(response.json()?).await?;
        Ok(json.as_string().unwrap_or_default())
    } else {
        Err(JsValue::from_str("Failed to fetch post"))
    }
}

#[component]
pub fn FetchComponent() -> impl IntoView {
    // Remove the `count` signal as it's not used for fetching API data
    // let (count, set_count) = create_signal(0);

    let post_data = create_resource(|| (), |_| async move { fetch_post().await.ok() });

    // // Adjusting view to display fetched post data
    let post_result = move || {
        post_data.get().as_ref().map_or_else(
            || "Fetching post...".into(),             // Used if Option is None
            |data| format!("Post fetched: {}", data), // Used if Option is Some
        )
    };

    view! {
        <p>
            {post_result}

        </p>
    }
}
