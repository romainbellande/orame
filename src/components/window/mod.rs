pub use leptos::*;
use leptos::{ev::mousedown, html::Div, leptos_dom::logging::console_log};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MouseEvent;

#[component]
pub fn Window(children: Children) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let header_node = create_node_ref::<Div>();

    let (pos1, set_pos1) = create_signal(0);
    let (pos2, set_pos2) = create_signal(0);
    let (pos3, set_pos3) = create_signal(0);
    let (pos4, set_pos4) = create_signal(0);

    let close_drag_element = Closure::<dyn FnMut()>::new(move || {
        let document = document();
        document.set_onmouseup(None);
        document.set_onmousemove(None);
    });

    let element_drag = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        event.prevent_default();
        set_pos1(pos3.get_untracked() - event.client_x());
        set_pos2(pos4.get_untracked() - event.client_y());
        set_pos3(event.client_x());
        set_pos4(event.client_y());

        let element = node_ref.get_untracked().unwrap();
        let new_x = element.offset_top() - pos2.get_untracked();
        let new_y = element.offset_left() - pos1.get_untracked();
        let _ = element
            .style("top", format!("{}px", new_x))
            .style("left", format!("{}px", new_y));
    });

    header_node.on_load(move |element: HtmlElement<Div>| {

        let _ = element.on(mousedown, move |event: MouseEvent| {
            event.prevent_default();
            set_pos3(event.client_x());
            set_pos4(event.client_y());

            let document = document();
            document.set_onmouseup(Some(close_drag_element.as_ref().unchecked_ref()));
            document.set_onmousemove(Some(element_drag.as_ref().unchecked_ref()));
        });
    });

    view! {
      <div node_ref=node_ref class="fixed top-1/2 left-1/2 transform -translate-y/2 -translate-x-2 bg-black rounded-lg w-96">
        <div node_ref=header_node class="cursor-move h-6 border-b border-solid border-s-slate-200 rounded-tr-lg rounded-tl-lg p-1 bg-slate-300 space-x-1 flex justify-end items-center">
          <button class="rounded-full bg-green-500 h-4 w-4"></button>
          <button class="rounded-full bg-yellow-500 h-4 w-4"></button>
          <button class="rounded-full bg-red-500 h-4 w-4"></button>
        </div>
        <div class="h-56">{ children() }</div>
      </div>
    }
}
