use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlElement};

use crate::constants::{
    COORDINATE_SELECTOR, CRIES, CURSOR_SELECTOR, DISPLAY_NONE, DRAG, DRAG_ENTER, DRAG_SELECTOR,
    LINK_SELECTOR, STYLE, TRASH, ZOOM,
};
use crate::util::{
    document, get_range, listen, node_list, query_selector, set_timeout, try_query_selector, window,
};

fn remove_drag_enter() {
    let coord = query_selector(COORDINATE_SELECTOR);
    let _ = coord.class_list().remove_1(DRAG_ENTER);
    let _ = coord.class_list().remove_1(TRASH);
}

fn reset() {
    remove_drag_enter();
    if let Ok(Some(drag)) = document().query_selector(DRAG_SELECTOR) {
        let _ = drag.class_list().remove_1(DRAG);
    }
}

fn delete() {
    if let Ok(Some(drag)) = document().query_selector(DRAG_SELECTOR) {
        let _ = drag.set_attribute(STYLE, DISPLAY_NONE);
    }
    reset();
}

fn bind_link(link: HtmlElement, cursor: HtmlElement, coord: HtmlElement) {
    let cursor_for_drag = cursor.clone();
    listen(link.as_ref(), "drag", move |event: DragEvent| {
        let _ = cursor_for_drag.set_attribute(
            STYLE,
            &format!(
                "transform: translate3d({}px,{}px,0);",
                event.client_x(),
                event.client_y()
            ),
        );
    });

    let link_for_start = link.clone();
    let cursor_for_start = cursor.clone();
    let coord_for_start = coord.clone();
    listen(link.as_ref(), "dragstart", move |_event: DragEvent| {
        let _ = coord_for_start.class_list().add_1(TRASH);
        let _ = cursor_for_start.class_list().remove_1(ZOOM);
        let clone = link_for_start
            .clone_node_with_deep(true)
            .expect("drag clone");
        let _ = link_for_start.class_list().add_1(DRAG);
        let _ = cursor_for_start.append_child(&clone);
    });

    listen(link.as_ref(), "dragend", move |_event: DragEvent| {
        set_timeout(reset, 100);
        remove_drag_enter();
        if let Some(cursor) = try_query_selector(CURSOR_SELECTOR)
            && let Ok(Some(project)) = cursor.query_selector(".project")
        {
            let _ = cursor.remove_child(&project);
        }
    });
}

pub(crate) struct Trash;

impl Trash {
    pub(crate) fn new() -> Self {
        let coord = query_selector(COORDINATE_SELECTOR);

        listen(coord.as_ref(), "dragover", move |event: DragEvent| {
            event.prevent_default();
        });

        let coord_for_enter = coord.clone();
        listen(coord.as_ref(), "dragenter", move |event: DragEvent| {
            event.prevent_default();
            let _ = coord_for_enter.class_list().add_1(DRAG_ENTER);
        });

        let coord_for_leave = coord.clone();
        listen(coord.as_ref(), "dragleave", move |event: DragEvent| {
            event.prevent_default();
            let _ = coord_for_leave.class_list().remove_1(DRAG_ENTER);
        });

        listen(coord.as_ref(), "drop", move |event: DragEvent| {
            event.prevent_default();
            let index = get_range(0.0, CRIES.len() as f64) as usize;
            if window().confirm_with_message(CRIES[index]).unwrap_or(false) {
                delete();
            } else {
                reset();
            }
        });

        let cursor = query_selector(CURSOR_SELECTOR);
        for link in node_list(LINK_SELECTOR) {
            bind_link(
                link.dyn_into().expect("draggable link"),
                cursor.clone(),
                coord.clone(),
            );
        }

        Self
    }
}
