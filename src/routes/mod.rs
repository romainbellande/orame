use leptos::*;
use leptos::leptos_dom::logging::console_log;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use futures::StreamExt;

mod home;
mod planets;

use crate::components::{header::Header, sidenav::SideNav};

use home::Home;
use ogame_core::{game::Game, protocol::Protocol};
use planets::PageID;
use crate::socket::Socket;

fn bytes_to_protocol(bytes: &[u8]) -> Protocol {
    serde_cbor::from_slice(bytes).unwrap()
}

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
pub fn AppRouter() -> impl IntoView {
    provide_meta_context();

    let context = create_rw_signal(Game::new());
    set_tick_interval(context);
    spawn_local(connect_socket(context));
    provide_context(context);

    view! {
      <Stylesheet id="leptos" href="/pkg/tailwind.css"/>

      <Router>
        <div>
          <Header />
          <SideNav />
          <main class="pt-28 pl-72 min-h-screen flex flex-col w-screen space-y-4">
            // all our routes will appear inside <main>
            <Routes>
              <Route path="/" view=Home/>
              <Route path="/planets/:id" view=PageID />
              <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
          </main>
        </div>
      </Router>
    }
}
