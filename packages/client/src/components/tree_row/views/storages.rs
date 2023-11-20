use std::collections::BTreeMap;

use leptos::*;

use ogame_core::storage::Storage;

use crate::components::tree_row::{IntoTreeItem, TreeItem};

pub struct StoragesTreeItem(pub BTreeMap<String, Storage>);

impl IntoTreeItem for StoragesTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <div class="grid grid-cols-4 gap-4 ml-8">
                <span> Storage id </span>
                <span> Structure id </span>
            </div>
        }
        .into_view();

        TreeItem {
            view,
            id: "Storages".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(_, storage)| StorageTreeItem(storage).into_tree_item())
                .collect(),
            collapsed: create_rw_signal(false),
        }
    }
}

pub struct StorageTreeItem(pub Storage);

impl IntoTreeItem for StorageTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <div class="grid grid-cols-4 gap-4 ml-4">
                <span> {self.0.id.clone()} </span>
                <span> {self.0.structure_id.clone()} </span>
            </div>

        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: vec![ItemsTreeItem(self.0.items.clone()).into_tree_item()],
            collapsed: create_rw_signal(true),
        }
    }
}

pub struct ItemsTreeItem(pub BTreeMap<String, usize>);

impl IntoTreeItem for ItemsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <div class="grid grid-cols-4 gap-4 ml-8">
                <span> Item id </span>
                <span> Amount </span>
            </div>
        }
        .into_view();

        TreeItem {
            view,
            id: "Items".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(name, amount)| ItemTreeItem(name, amount).into_tree_item())
                .collect(),
            collapsed: create_rw_signal(true),
        }
    }
}

pub struct ItemTreeItem(pub String, pub usize);

impl IntoTreeItem for ItemTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <div class="grid grid-cols-4 gap-4 ml-4">
                <span> {self.0.clone()} </span>
                <span> {self.1.clone()} </span>
            </div>

        }
        .into_view();

        TreeItem {
            view,
            id: self.0.clone(),
            children: vec![],
            collapsed: create_rw_signal(true),
        }
    }
}
