use super::Action;
use super::IntoContextMenuRows;

#[derive(Clone)]
pub struct ContextMenuRows {
    pub rows: Vec<Action>,
}

impl IntoContextMenuRows for ContextMenuRows {
    fn into_context_menu(&self) -> ContextMenuRows {
        self.clone()
    }
}
