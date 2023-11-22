use leptos::*;

pub mod views;

pub trait IntoTreeItem {
    fn into_tree_item(&self) -> TreeItem;
}

impl<F, I> IntoTreeItem for F
where
    F: Fn() -> I,
    I: IntoTreeItem,
{
    fn into_tree_item(&self) -> TreeItem {
        self().into_tree_item()
    }
}

impl IntoTreeItem for TreeItem {
    fn into_tree_item(&self) -> TreeItem {
        self.clone()
    }
}

#[derive(Clone, PartialEq)]
pub struct TreeItem {
    pub view: View,
    pub id: String,
    pub children: Vec<TreeItem>,
    pub collapsed: RwSignal<bool>,
}

#[component]
pub fn TreeRow<T>(
    tree_item: T,
    #[prop(default = 255)] depth: u8,
    #[prop(optional)] level: u8,
) -> impl IntoView
where
    T: IntoTreeItem + Clone + 'static,
{
    let tree_item = create_memo(move |_| tree_item.into_tree_item());

    if depth == 0 {
        return view! {<div></div>};
    }

    let is_collapsed = move || tree_item().collapsed;
    let toggle_collapsed =
        move || is_collapsed().update(|is_collapsed| *is_collapsed = !*is_collapsed);

    let arrow_classes = move || {
        if depth == 0 || level == 0 || tree_item().children.is_empty() {
            ""
        } else if (tree_item().collapsed)() {
            "arrow right"
        } else {
            "arrow down"
        }
    };

    let item_classes = move || match level {
        0 => "bg-gray-700",
        1 => "bg-gray-800",
        2 => "bg-gray-900",
        _ => "",
    };

    view! {
        <div class="flex flex-col">
            <div class=item_classes on:click=move |_| { if level > 0 { toggle_collapsed() } }>
                <span class="float-left">
                    <i class=arrow_classes> </i>
                </span>
                {move || tree_item().view}
            </div>
            <For
                each=move || tree_item().children
                key=move |child| child.id.clone()
                let:child
            >
                {
                    let child = store_value(child.clone());

                    view! {
                        <Show when=move || !is_collapsed()()>
                            <div class="ml-4">
                                <TreeRow tree_item=child depth={depth - 1} level={level + 1}/>
                            </div>
                        </Show>
                    }
                }
            </For>
        </div>
    }
}
