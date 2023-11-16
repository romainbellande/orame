pub use leptos::*;
use leptos::{ev::mousedown, html::Div, leptos_dom::logging::console_log};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{self, MouseEvent};

pub struct Drag {
    node_ref: NodeRef<Div>,
    handler_ref: NodeRef<Div>,
}

impl Drag {
    pub fn new() -> Self {
        let node_ref = create_node_ref::<Div>();
        let handler_ref = create_node_ref::<Div>();
        let (current_pos_x, set_current_pos_x) = create_signal(0);
        let (current_pos_y, set_current_pos_y) = create_signal(0);
        let (previous_pos_x, set_previous_pos_x) = create_signal(0);
        let (previous_pos_y, set_previous_pos_y) = create_signal(0);

        let close_drag_element = Closure::<dyn FnMut()>::new(move || {
            let document = document();
            document.set_onmouseup(None);
            document.set_onmousemove(None);
        });

        let element_drag = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
            event.prevent_default();
            set_current_pos_x(previous_pos_x.get_untracked() - event.client_x());
            set_current_pos_y(previous_pos_y.get_untracked() - event.client_y());
            set_previous_pos_x(event.client_x());
            set_previous_pos_y(event.client_y());

            let element = node_ref.get_untracked().unwrap();
            let new_x = element.offset_top() - current_pos_y.get_untracked();
            let new_y = element.offset_left() - current_pos_x.get_untracked();
            let _ = element
                .style("top", format!("{}px", new_x))
                .style("left", format!("{}px", new_y));
        });

        handler_ref.on_load(move |element: HtmlElement<Div>| {
            let _ = element.on(mousedown, move |event: MouseEvent| {
                event.prevent_default();
                set_previous_pos_x(event.client_x());
                set_previous_pos_y(event.client_y());

                let document = document();
                document.set_onmouseup(Some(close_drag_element.as_ref().unchecked_ref()));
                document.set_onmousemove(Some(element_drag.as_ref().unchecked_ref()));
            });
        });

        Self {
            node_ref,
            handler_ref,
        }
    }

    pub fn get_node_ref(&self) -> NodeRef<Div> {
        self.node_ref
    }

    pub fn get_handler_ref(&self) -> NodeRef<Div> {
        self.handler_ref
    }
}
