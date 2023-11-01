use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod home;
use home::Home;

mod planets;
use crate::components::{header::Header, sidenav::SideNav};
use planets::PageID;

#[component]
pub fn AppRouter() -> impl IntoView {
    provide_meta_context();

    view! {
      <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

      <Router>
        <div>
          <Header />
          <SideNav />
          <main class="pt-28 pl-72 min-h-screen flex flex-col w-screen space-y-4">
            // all our routes will appear inside <main>
            <Routes>
              <Route path="/" view=Home/>
              <Route path="/planets/:id" view=PageID />
              <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
          </main>
        </div>
      </Router>
    }
}
