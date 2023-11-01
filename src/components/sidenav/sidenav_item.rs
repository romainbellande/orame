use leptos::*;
use leptos_router::*;

#[component]
pub fn SidenavItem(href: &'static str, children: Children) -> impl IntoView {
    view! {
      <li class="group">
        <A href=href class="group-hover:text-primary group-hover:cursor-pointer flex space-x-4 items-center">
          <span>{children()}</span>
        </A>
      </li>
    }
}
