use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use web_sys::{Document, EventTarget, HtmlElement, Window};

use crate::constants::{A, BUTTON, CLASS, EMPTY, HASH, HREF, POOP, TYPE, YEAR_SELECTOR};

pub(crate) fn window() -> Window {
    web_sys::window().expect("window")
}

pub(crate) fn document() -> Document {
    window().document().expect("document")
}

pub(crate) fn listen<E, F>(target: &EventTarget, event_type: &str, handler: F)
where
    E: FromWasmAbi + JsCast + 'static,
    F: FnMut(E) + 'static,
{
    let callback = Closure::<dyn FnMut(E)>::new(handler);
    target
        .add_event_listener_with_callback(event_type, callback.as_ref().unchecked_ref())
        .expect("event listener");
    callback.forget();
}

pub(crate) struct PointerState {
    position: RefCell<Option<(f64, f64)>>,
    frame_requested: Cell<bool>,
}

impl PointerState {
    pub(crate) fn new() -> Rc<Self> {
        Rc::new(Self {
            position: RefCell::new(None),
            frame_requested: Cell::new(false),
        })
    }

    pub(crate) fn update<F>(self: Rc<Self>, x: f64, y: f64, render: F)
    where
        F: FnOnce(f64, f64) + 'static,
    {
        *self.position.borrow_mut() = Some((x, y));
        if self.frame_requested.replace(true) {
            return;
        }

        let state = self.clone();
        let callback = Closure::once(move || {
            state.frame_requested.set(false);
            if let Some((x, y)) = state.position.borrow_mut().take() {
                render(x, y);
            }
        });
        window()
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .expect("animation frame");
        callback.forget();
    }
}

pub(crate) fn node_list(selector: &str) -> Vec<web_sys::Element> {
    let nodes = document()
        .query_selector_all(selector)
        .expect("valid selector");
    (0..nodes.length())
        .filter_map(|index| nodes.item(index))
        .filter_map(|node| node.dyn_into().ok())
        .collect()
}

pub(crate) fn create_element(element_type: &str, class: &str) -> HtmlElement {
    let el: HtmlElement = document()
        .create_element(element_type)
        .expect("create element")
        .dyn_into()
        .expect("HTML element");
    if element_type == A {
        el.set_attribute(HREF, HASH).expect("href");
    }
    if element_type == BUTTON {
        el.set_attribute(TYPE, BUTTON).expect("button type");
    }
    el.set_attribute(CLASS, class).expect("class");
    el
}

pub(crate) fn get_hash() -> String {
    window()
        .location()
        .hash()
        .unwrap_or_default()
        .replace(HASH, EMPTY)
}

pub(crate) fn query_selector(selector: &str) -> HtmlElement {
    document()
        .query_selector(selector)
        .expect("valid selector")
        .expect(POOP)
        .dyn_into()
        .expect("HTML element")
}

pub(crate) fn try_query_selector(selector: &str) -> Option<HtmlElement> {
    document()
        .query_selector(selector)
        .ok()
        .flatten()
        .and_then(|el| el.dyn_into().ok())
}

pub(crate) fn get_range(start: f64, end: f64) -> f64 {
    start + js_sys::Math::random() * (end - start)
}

pub(crate) fn set_date() {
    let year = js_sys::Date::new_0().get_full_year();
    query_selector(YEAR_SELECTOR).set_text_content(Some(&year.to_string()));
}

pub(crate) fn set_timeout<F>(handler: F, milliseconds: i32)
where
    F: FnOnce() + 'static,
{
    let callback = Closure::once(handler);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            milliseconds,
        )
        .expect("timeout");
    callback.forget();
}
