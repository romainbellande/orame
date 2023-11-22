use leptos::*;

use super::FlightsWindow;
use super::ShipsWindow;
use super::StoragesWindow;
use super::UniverseWindow;
use super::WindowsContext;

#[component]
pub fn Windows() -> impl IntoView {
    provide_context(WindowsContext::new());

    view! {
        <ShipsWindow />
        <UniverseWindow />
        <FlightsWindow />
        <StoragesWindow />
    }
}
