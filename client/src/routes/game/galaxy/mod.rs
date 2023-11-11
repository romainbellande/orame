use crate::components::galaxy::GalaxyView;
use leptos::*;

#[component]
pub fn GalaxyPage() -> impl IntoView {
    view! {
        <div>
            <GalaxyView />
        </div>
    }
}
