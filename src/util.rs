use stdweb::traits::*;
use stdweb::web::{document, Date, HtmlElement, NodeList};

use stdweb::unstable::TryInto;

pub fn nl(selector: &str) -> NodeList {
    document().query_selector_all(selector).unwrap()
}

pub fn create_el(_type: &str, class: &str) -> HtmlElement {
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
        .to_string()
}

pub fn qs(selector: &str) -> HtmlElement {
    let t = document().query_selector(selector).unwrap();

    if t.is_none() {
        panic!("💩")
    }

    t.unwrap().try_into().unwrap()
}

pub fn get_range(a: f64, b: f64) -> f64 {
    js! (
      return (Math.random() * @{b}) + @{a};
    )
    .try_into()
    .unwrap()
}

pub fn set_date() {
    qs(".year").set_text_content(&Date::new().get_full_year().to_string());
}
