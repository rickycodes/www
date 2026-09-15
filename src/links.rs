use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent, Node};

use crate::constants::{
    BLANK, HIDDEN, INFO_LINKS_SELECTOR, INFO_SELECTOR, NAME, NOOPENER, NOT_PROJECT_SELECTOR,
    PROJECT_LINK_SELECTOR, REL, TARGET, TITLE,
};
use crate::util::{get_range, listen, node_list, query_selector};

fn show_info(attribute: &str, el: &HtmlElement, info: &HtmlElement) {
    if let Some(attr_value) = el.get_attribute(attribute) {
        info.set_text_content(Some(&attr_value));
        info.class_list().remove_1(HIDDEN).expect("info class");
    }
}

fn hide_info(info: &HtmlElement) {
    info.class_list().add_1(HIDDEN).expect("info class");
}

pub(crate) struct Links;

impl Links {
    fn shuffle_project_links() {
        let mut links: Vec<Node> = node_list(PROJECT_LINK_SELECTOR)
            .into_iter()
            .map(Into::into)
            .collect();

        if links.len() <= 1 {
            return;
        }

        for i in (1..links.len()).rev() {
            let j = get_range(0.0, (i + 1) as f64) as usize;
            links.swap(i, j);
        }

        let Some(parent) = links[0].parent_node() else {
            return;
        };
        for link in links {
            parent.append_child(&link).expect("project link");
        }
    }

    pub(crate) fn new() -> Self {
        let info = query_selector(INFO_SELECTOR);

        for link in node_list(INFO_LINKS_SELECTOR) {
            let el: HtmlElement = link.dyn_into().expect("info link");
            let info_for_over = info.clone();
            let el_for_over = el.clone();
            listen(el.as_ref(), "mouseover", move |_event: MouseEvent| {
                show_info(NAME, &el_for_over, &info_for_over);
                show_info(TITLE, &el_for_over, &info_for_over);
            });

            let info_for_out = info.clone();
            listen(el.as_ref(), "mouseout", move |_event: MouseEvent| {
                hide_info(&info_for_out);
            });
        }

        for link in node_list(NOT_PROJECT_SELECTOR) {
            let el: HtmlElement = link.dyn_into().expect("external link");
            el.set_attribute(TARGET, BLANK).expect("link target");
            el.set_attribute(REL, NOOPENER).expect("link rel");
        }

        Self::shuffle_project_links();
        Self
    }
}
