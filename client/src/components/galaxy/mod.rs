

use leptos::html::Canvas;
use leptos::*;
mod utils;
mod galaxy;
use galaxy::Galaxy;
mod star;


#[component]
pub fn GalaxyView() -> impl IntoView {
    let node_ref = create_node_ref::<Canvas>();

    node_ref.on_load(move |element| {
        let _ = element.on_mount(|_| {
            let mut galaxy = Galaxy::new("galaxy".to_string());
            galaxy.render();
        });

    });

    view! {
        <div class="bg-black">
            <canvas id="galaxy" class="w-screen h-screem" node_ref=node_ref on:click=move |_| {
                let mut galaxy = Galaxy::new("galaxy".to_string());
                galaxy.render();
            }></canvas>
        </div>
    }
}
