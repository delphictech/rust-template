mod components {
    pub mod app;
    pub mod fetch;
}

use components::app::App;

fn main() {
    leptos::mount_to_body(App)
}
