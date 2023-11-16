use super::ContextMenuRows;

pub trait IntoContextMenuRows {
    fn into_context_menu(&self) -> ContextMenuRows;
}

impl IntoContextMenuRows for () {
    fn into_context_menu(&self) -> ContextMenuRows {
        ContextMenuRows { rows: vec![] }
    }
}
