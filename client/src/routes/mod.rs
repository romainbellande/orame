use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod portal;
use portal::Portal;

mod game;
use crate::global_state::GlobalState;
use game::{GamePage, HomePage, PlanetIDPage};

#[component]
pub fn AppRouter() -> impl IntoView {
    provide_meta_context();
    let global_state = create_rw_signal(GlobalState::default());
    provide_context(global_state);

    view! {
      // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

      <Router>
        <Routes>
          <Route path="/portal" view=Portal />
          <Route path="/" view=GamePage>
            <Route path="" view=HomePage />
            <Route path="/planets/:id" view=PlanetIDPage />
          </Route>
          <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
      </Router>
    }
}
