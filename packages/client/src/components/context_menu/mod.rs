use leptos::{html::Div, *};

mod action;
mod context;
mod context_menu_rows;
mod into_context_menu_rows;
pub mod views;

pub use action::Action;
pub use context::*;
pub use context_menu_rows::*;
pub use into_context_menu_rows::*;

#[component]
pub fn ContextMenu() -> impl IntoView {
    provide_context(create_rw_signal(ContextMenuContext::new()));
    let context = expect_context::<RwSignal<ContextMenuContext>>();

    let element = create_node_ref::<Div>();

    create_effect(move |_| {
        let element = element.get_untracked().unwrap();
        let _ = element
            .style("top", format!("{}px", context().top))
            .style("left", format!("{}px", context().left));
    });

    let row_event = move |row: Action| {
        let inner_event = row.to_click_event();
        move |ev| {
            context.update(|context| context.visible = false);
            inner_event(ev);
        }
    };

    view! {
        <div node_ref=element class="absolute z-50">
            <Show
                when=move || context().visible
            >
                <div class="bg-gray-800 rounded shadow-lg">
                    <ul class="list-reset">
                        <For
                            each=move || context().rows.into_context_menu().rows
                            key=move |row| row.to_string()
                            let:row
                        >
                            <li class="hover:bg-gray-700">
                                <button class="block px-4 py-2 text-white no-underline" on:click=row_event(row.clone())>
                                    {row.to_string()}
                                </button>
                            </li>
                        </For>
                    </ul>
                </div>
            </Show>
        </div>
    }
}
