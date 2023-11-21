use futures::{channel::mpsc::Receiver, StreamExt};
use leptos::*;
use leptos_router::Outlet;
use ogame_core::{game::Game, protocol::Protocol};
use reqwasm::http::Request;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{
    components::{context_menu::ContextMenu, sidenav::SideNav, window::Windows},
    error::*,
    utils::{GameWrapper, Socket},
};

mod home;

pub use home::HomePage;

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

pub async fn get_game_data() -> Result<ogame_core::GameData> {
    let data = Request::get("/public/game_data.cbor")
        .send()
        .await?
        .binary()
        .await?;

    let game_data: ogame_core::GameData = serde_cbor::from_slice(&data[..])?;
    *crate::GAME_DATA.write().unwrap() = game_data.clone();

    Ok(game_data)
}

pub async fn init_game(game: RwSignal<GameWrapper>, new_game: Game) -> Result<()> {
    let game_data = get_game_data().await?;

    game.update(|game| {
        **game = new_game;
        game.game_data = game_data;
    });

    set_tick_interval(game);

    Ok(())
}

pub async fn connect_socket(
    game: RwSignal<GameWrapper>,
    mut game_rx: Receiver<Protocol>,
    ready: RwSignal<bool>,
) {
    let mut ws: Socket<Protocol> = Socket::connect("ws://localhost:8080/ws").await;

    let mut recv = ws.take_receiver().unwrap();

    spawn_local(async move {
        while let Some(msg) = recv.next().await {
            if let Protocol::Game(new_game) = msg {
                init_game(game, new_game).await.unwrap();
                ready.set(true);
            } else {
                game.update(|game| {
                    game.process_message(msg).unwrap();
                });
            }
        }
    });

    while let Some(msg) = game_rx.next().await {
        ws.send(msg).await.unwrap();
    }
}

#[component]
pub fn GamePage() -> impl IntoView {
    let (tx, rx) = futures::channel::mpsc::channel(1);
    let ready = create_rw_signal(false);

    let game_context = create_rw_signal(crate::utils::GameWrapper::new(Game::new(), tx));

    spawn_local(connect_socket(game_context, rx, ready));

    provide_context(game_context);

    view! {
         // <div class="flex max-w-full max-h-full font-shentox" oncontextmenu="event.preventDefault();">
        <Show
            when={ready}
        >
             <div class="text-white flex max-w-full max-h-full font-shentox">
                <ContextMenu />
                <Windows />
                <SideNav />
                <main class="min-h-screen flex flex-col w-screen space-y-4 bg-gray-900 bg-map opacity-30" >
                    <Outlet />
                </main>
            </div>
        </Show>
    }
}
