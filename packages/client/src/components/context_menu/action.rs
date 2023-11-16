use leptos::*;
use web_sys::MouseEvent;

use crate::components::window::WindowsContext;

#[derive(Clone)]
pub enum Action {
    OpenWindow(String),
    UpgradeBuilding(String, usize),
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::OpenWindow(name) => format!("Open {}", name),
            Action::UpgradeBuilding(building_type, level) => {
                format!("Upgrade {} to level {}", building_type, level + 1)
            }
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
            Action::UpgradeBuilding(building_type, level) => Box::new(move |ev: MouseEvent| {
                ev.prevent_default();
            })
                as Box<dyn Fn(MouseEvent)>,
        }
    }
}
