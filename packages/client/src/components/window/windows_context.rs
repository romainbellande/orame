use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use leptos::{leptos_dom::logging::console_log, *};

#[derive(Clone)]
pub struct WindowsContext {
    pub windows: Rc<RefCell<BTreeMap<String, RwSignal<bool>>>>,
}

impl WindowsContext {
    pub fn new() -> Self {
        Self {
            windows: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn register(&mut self, id: String) -> RwSignal<bool> {
        let signal = create_rw_signal(false);
        self.windows.borrow_mut().insert(id, signal.clone());
        signal
    }

    pub fn toggle(&self, id: &str) {
        console_log("Toggling window>>>>");
        if let Some(window) = self.windows.borrow().get(id) {
            console_log("Toggling window");
            window.update(|v| *v = !*v);
        }
    }

    pub fn get_visible(&self, id: &str) -> RwSignal<bool> {
        if let Some(window) = self.windows.borrow().get(id) {
            *window
        } else {
            panic!("Window {} not found", id);
        }
    }
}
