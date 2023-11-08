use leptos::*;

#[component]
pub fn TooltipProvider(children: Children) -> impl IntoView {
  view! {
    <div class="tooltip">
      {children()}
    </div>
  }
}

#[component]
pub fn TooltipContent(children: Children) -> impl IntoView {
  view! {
    <div class="tooltip-content h-min w-min px-4 py-2">
      {children()}
    </div>
  }
}
