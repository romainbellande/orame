use leptos::*;
use leptos_router::Outlet;
mod home;
mod planets;
use crate::components::{header::Header, sidenav::SideNav};
use crate::utils::Socket;
use futures::StreamExt;
pub use home::HomePage;
use ogame_core::{game::Game, protocol::Protocol};
pub use planets::PlanetIDPage;
use wasm_bindgen::{prelude::Closure, JsCast};

fn set_tick_interval(game: RwSignal<Game>) {
    let cb = Closure::wrap(Box::new(move || {
        game.update(|game| {
            game.tick().unwrap();
        });
    }) as Box<dyn FnMut()>);

    let window = web_sys::window().unwrap();
    let _interval_id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 1_000)
        .unwrap();
    cb.forget(); // leak the closure
}

pub async fn connect_socket(game: RwSignal<Game>) {
    let mut ws: Socket<Protocol> = Socket::connect("ws://localhost:8080/ws").await;

    let mut recv = ws.take_receiver().unwrap();

    while let Some(msg) = recv.next().await {
        game.update(|game| {
            game.process_message(msg).unwrap();
        });
    }
}

#[component]
pub fn GamePage() -> impl IntoView {
    let game_context = create_rw_signal(Game::new());
    set_tick_interval(game_context);
    spawn_local(connect_socket(game_context));

    provide_context(game_context);

    view! {
      <div>
      <Header />
      <SideNav />
      <main class="pt-28 pl-72 min-h-screen flex flex-col w-screen space-y-4">
        <Outlet />
      </main>
    </div>
    }
}
