use leptos::*;
use crate::components::galaxy::GalaxyView;

#[component]
pub fn GalaxyPage() -> impl IntoView {
    view! {
        <div>
            <GalaxyView />
        </div>
    }
}
