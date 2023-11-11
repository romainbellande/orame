use futures::channel::mpsc::Receiver;
use leptos::*;
use leptos_router::Outlet;
use crate::components::{header::Header, sidenav::SideNav};
use crate::utils::{GameWrapper, Socket};
use futures::StreamExt;
use ogame_core::{game::Game, protocol::Protocol};
use wasm_bindgen::{prelude::Closure, JsCast};

mod home;
pub use home::HomePage;

mod planets;
pub use planets::{PlanetIDPage, PlanetsPage};

mod galaxy;
pub use galaxy::GalaxyPage;

fn set_tick_interval(game: RwSignal<GameWrapper>) {
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

pub async fn connect_socket(game: RwSignal<GameWrapper>, mut game_rx: Receiver<Protocol>) {
    let mut ws: Socket<Protocol> = Socket::connect("ws://localhost:8080/ws").await;

    let mut recv = ws.take_receiver().unwrap();

    spawn_local(async move {
        while let Some(msg) = recv.next().await {
            game.update(|game| {
                game.process_message(msg).unwrap();
            });
        }
    });

    while let Some(msg) = game_rx.next().await {
        ws.send(msg).await.unwrap();
    }
}

#[component]
pub fn GamePage() -> impl IntoView {
    let (tx, rx) = futures::channel::mpsc::channel(1);

    let game_context = create_rw_signal(crate::utils::GameWrapper::new(Game::new(), tx));

    set_tick_interval(game_context);

    spawn_local(connect_socket(game_context, rx));

    provide_context(game_context);

    view! {
         <div class="max-w-full max-h-full">
            <Header />
            <SideNav />
            <main class="pt-28 pl-72 min-h-screen flex flex-col w-screen space-y-4 bg-slate-800">
                <Outlet />
            </main>
        </div>
    }
}
