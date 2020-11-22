use stdweb::traits::*;
use stdweb::web::{document, Date, HtmlElement, NodeList};

use rand::Rng;
use stdweb::unstable::TryInto;

pub fn node_list(selector: &str) -> NodeList {
    document().query_selector_all(selector).unwrap()
}

pub fn create_element(_type: &str, class: &str) -> HtmlElement {
    let el = document().create_element(_type).unwrap();
    el.set_attribute("class", class).unwrap();
    el.try_into().unwrap()
}

pub fn get_hash() -> String {
    document()
        .location()
        .unwrap()
        .hash()
        .unwrap()
        .replace("#", "")
}

pub fn query_selector(selector: &str) -> HtmlElement {
    let t = document().query_selector(selector).unwrap();

    if t.is_none() {
        panic!("💩")
    }

    t.unwrap().try_into().unwrap()
}

pub fn get_range(start: f64, end: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(start, end) as f64
}

pub fn set_date() {
    query_selector(".year").set_text_content(&Date::new().get_full_year().to_string());
}
