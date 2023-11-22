use leptos::*;

mod components;
mod routes;
mod utils;

use routes::AppRouter;

pub fn start() {
    mount_to_body(|| view! { <AppRouter /> })
}
