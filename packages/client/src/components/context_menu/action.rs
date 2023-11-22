use leptos::*;
use web_sys::MouseEvent;

use crate::components::window::WindowsContext;

#[derive(Clone)]
pub enum Action {
    OpenWindow(String),
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::OpenWindow(name) => format!("Open {}", name),
        }
    }
}

impl Action {
    pub fn to_click_event(&self) -> Box<dyn Fn(MouseEvent)> {
        let windows = expect_context::<RwSignal<WindowsContext>>();

        match self.clone() {
            Action::OpenWindow(name) => Box::new(move |ev: MouseEvent| {
                ev.prevent_default();
                windows().toggle(&name);
            }) as Box<dyn Fn(MouseEvent)>,
        }
    }
}
