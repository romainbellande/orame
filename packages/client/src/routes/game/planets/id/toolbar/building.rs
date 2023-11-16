use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{building_type::BuildingType, planet::Planet};
use uuid::Uuid;
use web_sys::MouseEvent;

use crate::{components::window::Window, data::building::BuildingConfig, utils::GameWrapper};

use super::ui::ToolbarUI;

#[derive(Clone)]
pub struct Building {
    pub id: Uuid,
    pub show: ReadSignal<bool>,
    pub set_show: WriteSignal<bool>,
    pub config: BuildingConfig,
    pub level: usize,
}

impl From<BuildingType> for Building {
    fn from(value: BuildingType) -> Self {
        let config = BuildingConfig::from(value);
        let (show, set_show) = create_signal(false);
        Self {
            id: Uuid::new_v4(),
            show,
            set_show,
            config,
            level: 0,
        }
    }
}

impl Building {
    pub fn toggle_show(&self) {
        self.set_show.set(!self.show.get_untracked());
    }

    pub fn set_level(&mut self, level: usize) -> Self {
        self.level = level;
        self.clone()
    }

    pub fn get_level_from_planet(&self, planet: &Planet) -> usize {
        planet.building_level(self.config.building_type.clone())
    }

    pub fn get_type(&self) -> BuildingType {
        self.config.building_type.clone()
    }
}

/* #[component]
pub fn BuildingWindow(
    building: ReadSignal<Building>,
    planet: Signal<Planet>,
    ui: ReadSignal<ToolbarUI>,
) -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let on_upgrade = move |building: Building, planet: Planet| {
        move |_| {
            console_log(format!("upgrade building: {:?}", building.get_type()).as_str());
            state.update(|state| {
                if let Err(e) = state.upgrade_building(planet.id.clone(), building.get_type()) {
                    console_log(format!("Error upgrade building: {:?}", e).as_str());
                }
            });
        }
    };

    let level = move || with!(|building, planet| building.get_level_from_planet(planet));

    view! {
          <Window title="Building" on_show=move |value: bool| ui().set_building_visibility(building().config.building_type, value)>
            <div class="space-y-4">
              <h3><span>{ building().config.name }</span><span>"level " {level}</span></h3>
              <div class=format!("sprite sprite_large building {}", building().config.class)></div>
              <p class="text-xs max-w-sm">{ building().config.description }</p>
              <div class="flex justify-end">

                <button type="button" on:click=on_upgrade(building(), planet()) class="rounded bg-indigo-500 p-2 px-4 text-white hover:bg-orange-500">"upgrade"</button>
              </div>
            </div>
          </Window>
    }
} */

#[component]
pub fn BuildingTile(
    building: ReadSignal<Building>,
    #[prop(into)] on_toggle: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
      <li class=format!("w-24 h-24 relative hover:brightness-125 cursor-pointer icon sprite sprite_medium medium {}", building.get().config.class) on:click=on_toggle >
      </li>
    }
}
