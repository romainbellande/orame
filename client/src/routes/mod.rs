use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod portal;
use portal::Portal;

mod game;
use game::{GamePage, HomePage, PlanetIDPage, PlanetsPage, GalaxyPage};

#[component]
pub fn AppRouter() -> impl IntoView {
    provide_meta_context();

    view! {
      // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

      <Router>
        <Routes>
          <Route path="/portal" view=Portal />
          <Route path="/" view=GamePage>
            <Route path="" view=HomePage />
            <Route path="/planets" view=PlanetsPage />
            <Route path="/planets/:id" view=PlanetIDPage />
            <Route path="/galaxy" view=GalaxyPage />
          </Route>
          <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
      </Router>
    }
}
