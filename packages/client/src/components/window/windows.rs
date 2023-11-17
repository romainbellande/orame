use leptos::*;

/* use super::BuildingsByPlanetWindow;
use super::PlanetsWindow;
use super::ShipsByPlanetWindow; */
use super::ShipsWindow;
use super::StoragesWindow;
use super::UniverseWindow;
use super::WindowsContext;

#[component]
pub fn Windows() -> impl IntoView {
    provide_context(create_rw_signal(WindowsContext::new()));

    view! {
        /* <PlanetsWindow />
        <BuildingsByPlanetWindow />
        <ShipsByPlanetWindow /> */
        <ShipsWindow />
        <UniverseWindow />
        <StoragesWindow />
    }
}
