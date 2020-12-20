use stdweb::traits::*;
use stdweb::web::{document, Date, HtmlElement, NodeList};

use rand::Rng;
use stdweb::unstable::TryInto;

use crate::constants::{CLASS, EMPTY, HASH, POOP, YEAR_SELECTOR};

pub(crate) fn node_list(selector: &str) -> NodeList {
    document().query_selector_all(selector).unwrap()
}

pub(crate) fn create_element(_type: &str, class: &str) -> HtmlElement {
    let el = document().create_element(_type).unwrap();
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
    let target = document().query_selector(selector).unwrap();

    if target.is_none() {
        panic!(POOP)
    }

    target.unwrap().try_into().unwrap()
}

pub(crate) fn get_range(start: f64, end: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(start, end) as f64
}

pub(crate) fn set_date() {
    query_selector(YEAR_SELECTOR).set_text_content(&Date::new().get_full_year().to_string());
}
