use std::cell::{Cell, RefCell};
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::web::{document, window, Date, HtmlElement, NodeList};

use stdweb::unstable::TryInto;

use crate::constants::{A, BUTTON, CLASS, EMPTY, HASH, HREF, POOP, TYPE, YEAR_SELECTOR};

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
        window().request_animation_frame(move |_| {
            state.frame_requested.set(false);
            if let Some((x, y)) = state.position.borrow_mut().take() {
                render(x, y);
            }
        });
    }
}

pub(crate) fn node_list(selector: &str) -> NodeList {
    document().query_selector_all(selector).unwrap()
}

pub(crate) fn create_element(element_type: &str, class: &str) -> HtmlElement {
    let el = document().create_element(element_type).unwrap();
    if element_type == A {
        el.set_attribute(HREF, HASH).unwrap();
    }
    if element_type == BUTTON {
        el.set_attribute(TYPE, BUTTON).unwrap();
    }
    el.set_attribute(CLASS, class).unwrap();
    el.try_into().unwrap()
}

pub(crate) fn get_hash() -> String {
    document()
        .location()
        .unwrap()
        .hash()
        .unwrap()
        .replace(HASH, EMPTY)
}

pub(crate) fn query_selector(selector: &str) -> HtmlElement {
    document()
        .query_selector(selector)
        .unwrap()
        .expect(POOP)
        .try_into()
        .unwrap()
}

pub(crate) fn try_query_selector(selector: &str) -> Option<HtmlElement> {
    document()
        .query_selector(selector)
        .ok()
        .and_then(|el| el)
        .and_then(|el| el.try_into().ok())
}

pub(crate) fn get_range(start: f64, end: f64) -> f64 {
    let rand: f64 = js!( return Math.random(); ).try_into().unwrap();
    start + rand * (end - start)
}

pub(crate) fn set_date() {
    query_selector(YEAR_SELECTOR).set_text_content(&Date::new().get_full_year().to_string());
}
