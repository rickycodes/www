use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;
use crate::util::{node_list, query_selector};

use stdweb::web::event::{MouseOutEvent, MouseOverEvent};

use crate::constants::{
    BLANK, HIDDEN, INFO_LINKS_SELECTOR, INFO_SELECTOR, NAME, NOOPENER, NOT_PROJECT_SELECTOR, REL,
    TARGET, TITLE,
};

pub(crate) fn show_info(attribute: &str, el: &HtmlElement, info: &HtmlElement) {
    if let Some(attr_value) = el.get_attribute(attribute) {
        info.set_text_content(&attr_value);
        info.class_list().remove(HIDDEN).unwrap();
    }
}

pub(crate) fn hide_info(info: &HtmlElement) {
    info.class_list().add(HIDDEN).unwrap();
}

pub(crate) struct Links;

impl Links {
    pub(crate) fn new() -> Self {
        let info = query_selector(INFO_SELECTOR);

        for link in node_list(INFO_LINKS_SELECTOR) {
            let el: HtmlElement = link.clone().try_into().unwrap();
            link.add_event_listener(enclose!( (el, info) move |_event: MouseOverEvent| {
              self::show_info(NAME, &el, &info);
              self::show_info(TITLE, &el, &info);
            }));

            link.add_event_listener(enclose!( (info) move |_event: MouseOutEvent| {
              self::hide_info(&info);
            }));
        }

        for link in node_list(NOT_PROJECT_SELECTOR) {
            // console!(log, &link);
            let el: HtmlElement = link.try_into().unwrap();
            el.set_attribute(TARGET, BLANK).unwrap();
            el.set_attribute(REL, NOOPENER).unwrap();
        }

        Self
    }
}
