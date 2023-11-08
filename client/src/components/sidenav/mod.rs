use leptos::*;

mod sidenav_item;
use sidenav_item::SidenavItem;

#[component]
pub fn SideNav() -> impl IntoView {
    view! {
      <nav class="pt-24 w-72 h-screen fixed space-y-4 mt-24 px-4 shadow bg-white">
        <div class="card w-full bg-base-100 shadow px-8 py-4">
          <ul class="space-y-4">
            <SidenavItem href="/" >home</SidenavItem>
            <SidenavItem href="/planets">"Planets"</SidenavItem>
          </ul>
        </div>
      </nav>

    }
}
