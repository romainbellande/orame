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

#[derive(Clone)]
pub struct TreeItem {
    pub view: View,
    pub id: String,
    pub children: Vec<TreeItem>,
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
    let tree_item = store_value(move || tree_item.into_tree_item());
    let (collapsed, set_collapsed) = create_signal(level > 0);

    if depth == 0 {
        return view! {<div></div>};
    }

    let arrow_classes = move || {
        if depth == 0 || level == 0 || tree_item()().children.is_empty() {
            ""
        } else if collapsed() {
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
            <div class=item_classes on:click=move |_| { if level > 0 {set_collapsed(!collapsed())}}>
                <i class=arrow_classes> </i>
                {move || tree_item()().view}
            </div>
            <For
                each=move || tree_item()().children
                key=move |child| child.id.clone()
                let:child
            >
                {
                    let child = store_value(child.clone());

                    view! {
                        <Show when=move || !collapsed()>
                            <div class="ml-4 pt-1">
                                <TreeRow tree_item=child() depth={depth - 1} level={level + 1}/>
                            </div>
                        </Show>
                    }
                }
            </For>
        </div>
    }
}
