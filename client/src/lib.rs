use leptos::*;
mod components;
mod routes;
mod utils;
mod data;

use routes::AppRouter;

pub fn start() {
    mount_to_body(|| view! { <AppRouter /> })
}
