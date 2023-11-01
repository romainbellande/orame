use leptos::*;

mod routes;
use routes::AppRouter;
mod components;

pub fn start() {
    mount_to_body(|| view! { <AppRouter /> })
}
