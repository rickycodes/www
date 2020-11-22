use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;
use util::{nl, qs};

use stdweb::web::event::{MouseOutEvent, MouseOverEvent};

pub struct Links();

pub fn show_info(str: &str, el: HtmlElement, info: HtmlElement) {
    let attr = el.get_attribute(str);
    if attr != None {
        let attr_str = attr.unwrap();
        info.set_text_content(&attr_str);
        info.class_list().remove("hidden").unwrap();
    }
}

pub fn hide_info(info: HtmlElement) {
    info.class_list().add("hidden").unwrap();
}

impl Links {
    pub fn new() -> Links {
        let info = qs(".info");

        for link in nl(".content a[title], .content label[name]") {
            let el: HtmlElement = link.clone().try_into().unwrap();
            link.add_event_listener(enclose!( (el, info) move |_event: MouseOverEvent| {
              show_info("name", el.clone(), info.clone());
              show_info("title", el.clone(), info.clone());
            }));

            link.add_event_listener(enclose!( (info) move |_event: MouseOutEvent| {
              hide_info(info.clone());
            }));
        }

        Links()
    }
}
