use crate::utils::{classnames, Classname, Drag};

pub use leptos::*;

#[component]
pub fn Window<F: Fn(bool) + 'static>(
    children: Children,
    on_show: F,
    title: &'static str,
    #[prop(default = 24)]
    width: usize
) -> impl IntoView {
    let (fullscreen, set_fullscreen) = create_signal(false);
    let (collapsed, set_collapsed) = create_signal(false);
    let drag = Drag::new();

    let node_ref = drag.get_node_ref();
    let handler_node = drag.get_handler_ref();

    let on_fullscreen_toggle = move |_| {
        if !fullscreen() {
            let _ = node_ref
                .get_untracked()
                .unwrap()
                .style("top", "0px")
                .style("left", "0px")
                .style("width", "")
                .style("height", "");
        }

        set_fullscreen(!fullscreen.get());
    };

    let on_close = move |_| {
        on_show(false);
    };

    let on_collapsed = move |_| {
        set_collapsed(!collapsed.get());
    };

    let root_classes = create_memo(move |_| {
        let fullscreen_class = if fullscreen() {
            "fixed top-0 left-0 w-screen h-screen"
        } else {
            "top-1/2 left-1/2 transform -translate-y/2 -translate-x-2"
        };

        let resize_class = if fullscreen() || collapsed() {
            "resize-none"
        } else {
            "resize"
        };

        let collapsed_class = if collapsed() { "" } else { "overflow-scroll" };

        classnames(vec![
            Classname::String("fixed bg-black rounded-lg".to_string()),
            Classname::String(fullscreen_class.to_string()),
            Classname::String(resize_class.to_string()),
            Classname::String(collapsed_class.to_string()),
        ])
    });

    let content_classes = create_memo(move |_| {
        let collapsed_class = if collapsed() {
            "max-h-0 p-0"
        } else {
            "p-4"
        };

        classnames(vec![
            Classname::String("transition-max-height duration-300 overflow-hidden".to_string()),
            Classname::String(collapsed_class.to_string()),
        ])
    });

    view! {
      <div node_ref=node_ref class=root_classes style:width=format!("{width}rem")>
        <div node_ref=handler_node class="cursor-move border-b border-solid border-s-slate-900 rounded-tr-lg rounded-tl-lg p-1 bg-slate-900 space-x-1 flex justify-between items-center">
          <div class="text-slate-300">{ title }</div>
          <div class="space-x-2">
            <button class="rounded-full bg-green-500 h-4 w-4" on:click=on_collapsed></button>
            <button class="rounded-full bg-yellow-500 h-4 w-4" on:click=on_fullscreen_toggle></button>
            <button class="rounded-full bg-red-500 h-4 w-4" on:click=on_close></button>
          </div>
        </div>
        <div class=content_classes>{ children() }</div>
      </div>
    }
}
