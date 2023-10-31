use leptos::*;
use leptos_router::*;
use leptos_meta::*;

mod home;
use home::Home;

#[component]
pub fn AppRouter() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

    <Router>
      <nav>
        "nav"
      </nav>
      <main>
        // all our routes will appear inside <main>
        <Routes>
          <Route path="/" view=Home/>
          <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
      </main>
    </Router>
  }
}
