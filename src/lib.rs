use leptos::*;

mod routes;
use routes::AppRouter;

pub fn start() {
  mount_to_body(|| view! { <AppRouter /> })
}
