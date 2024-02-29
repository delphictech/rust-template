mod components {
    pub mod app;
}

use components::app::App;

fn main() {
    leptos::mount_to_body(App)
}
