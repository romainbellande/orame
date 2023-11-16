use std::rc::Rc;

use web_sys::MouseEvent;

use super::IntoContextMenuRows;

#[derive(Clone)]
pub struct ContextMenuContext {
    pub visible: bool,
    pub rows: Rc<dyn IntoContextMenuRows>,
    pub top: i32,
    pub left: i32,
}

impl ContextMenuContext {
    pub fn new() -> Self {
        Self {
            visible: false,
            rows: Rc::new(()),
            top: 0,
            left: 0,
        }
    }

    pub fn show<T: IntoContextMenuRows + 'static>(&mut self, t: T, ev: MouseEvent) {
        self.visible = true;
        self.rows = Rc::new(t);
        self.top = ev.client_y();
        self.left = ev.client_x();
    }
}
