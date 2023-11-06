use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{building_type::BuildingType, planet::Planet};
use uuid::Uuid;
use web_sys::MouseEvent;

use crate::{components::window::Window, data::building::BuildingConfig};
use ogame_core::game::Game;

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

    pub fn get_type(&self) -> BuildingType {
        self.config.building_type.clone()
    }
}

#[component]
pub fn BuildingWindow(building: ReadSignal<Building>, planet: Signal<Planet>) -> impl IntoView {
    let state = expect_context::<RwSignal<Game>>();

    let on_upgrade = move |building: Building, planet: Planet| {
        move |_| {
            console_log(format!("upgrade building: {:?}", building.get_type()).as_str());
            state.update(|state| {
                if let Err(e) = state.upgrade_building(planet.id(), building.get_type()) {
                    console_log(format!("Error upgrade building: {:?}", e).as_str());
                }
            });
        }
    };

    view! {
      <Show when=building().show>
          <Window title="Building" on_show=building().set_show>
            <div class="space-y-4">
              <h3><span>{ building().config.name }</span><span>"level" {building.get().level}</span></h3>
              <div class=format!("sprite sprite_large building {}", building().config.class)></div>
              <p class="text-xs">{ building().config.description }</p>
              <div class="flex justify-end">

                <button type="button" on:click=on_upgrade(building(), planet()) class="rounded bg-indigo-500 p-2 px-4 text-white hover:bg-orange-500">"upgrade"</button>
              </div>
            </div>
          </Window>
        </Show>
    }
}

#[component]
pub fn BuildingTile(building: ReadSignal<Building>) -> impl IntoView {
    view! {
      <li class=format!("w-24 h-24 relative hover:brightness-125 cursor-pointer icon sprite sprite_medium medium {}", building.get().config.class) on:click=move |_| { building.get().toggle_show()  }>
      </li>
    }
}
