use leptos::html::{Canvas};
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use web_sys::{wasm_bindgen::JsValue};

#[component]
pub fn Galaxy() -> impl IntoView {
    let node_ref = create_node_ref::<Canvas>();

    node_ref.on_load(move |canvas| {
        canvas.set_width(800);
        canvas.set_height(600);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_fill_style(&JsValue::from_str("yellow"));
        context.begin_path();
        context
            .arc(400.0, 300.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();

        // Draw the planet
        context.set_fill_style(&JsValue::from_str("blue"));
        context.begin_path();
        context
            .arc(550.0, 300.0, 20.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
    });

    view! {
        <div>
            <canvas id="galaxy" node_ref=node_ref></canvas>
        </div>
    }
}
