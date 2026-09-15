use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};

use crate::constants::{CLOSE, CLOSE_SELECTOR, CURSOR_SELECTOR, CURSORS_ATTRIBUTES, STYLE};
use crate::util::{PointerState, listen, node_list, query_selector, window};

struct Cursor;

impl Cursor {
    fn new(el: &HtmlElement, cursor: &HtmlElement, classname: &'static str) -> Self {
        let cursor_for_over = cursor.clone();
        listen(el.as_ref(), "mouseover", move |_event: MouseEvent| {
            cursor_for_over
                .class_list()
                .add_1(classname)
                .expect("cursor class");
        });

        let cursor_for_out = cursor.clone();
        listen(el.as_ref(), "mouseout", move |_event: MouseEvent| {
            cursor_for_out
                .class_list()
                .remove_1(classname)
                .expect("cursor class");
        });

        Self
    }
}

fn set_cursor_coordinates(el: &HtmlElement, x: f64, y: f64) {
    el.set_attribute(STYLE, &format!("transform: translate3d({x}px,{y}px,0);"))
        .expect("cursor style");
}

pub(crate) struct Cursors;

impl Cursors {
    pub(crate) fn new() -> Self {
        let cursor_element = query_selector(CURSOR_SELECTOR);
        let close = query_selector(CLOSE_SELECTOR);
        let state = PointerState::new();

        Cursor::new(&close, &cursor_element, CLOSE);

        for cursor_attribute in &CURSORS_ATTRIBUTES {
            for element in node_list(cursor_attribute.selector) {
                let element: HtmlElement = element.dyn_into().expect("cursor target");
                Cursor::new(&element, &cursor_element, cursor_attribute.classname);
            }
        }

        listen(window().as_ref(), "mousemove", move |event: MouseEvent| {
            let cursor = cursor_element.clone();
            let state = state.clone();
            state.update(
                f64::from(event.client_x()),
                f64::from(event.client_y()),
                move |x, y| set_cursor_coordinates(&cursor, x, y),
            );
        });

        Self
    }
}
