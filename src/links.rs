use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent, Node};

use crate::constants::{
    BLANK, EXPECT_INFO_CLASS, HIDDEN, INFO_LINKS_SELECTOR, INFO_SELECTOR, MOUSEOUT, MOUSEOVER,
    NAME, NOOPENER, NOT_PROJECT_SELECTOR, PROJECT_LINK_SELECTOR, REL, TARGET, TITLE,
};
use crate::util::{get_range, listen, node_list, query_selector};

fn show_info(attribute: &str, el: &HtmlElement, info: &HtmlElement) {
    if let Some(attr_value) = el.get_attribute(attribute) {
        info.set_text_content(Some(&attr_value));
        info.class_list().remove_1(HIDDEN).expect(EXPECT_INFO_CLASS);
    }
}

fn hide_info(info: &HtmlElement) {
    info.class_list().add_1(HIDDEN).expect(EXPECT_INFO_CLASS);
}

fn bind_info_link(link: HtmlElement, info: HtmlElement) {
    let info_for_over = info.clone();
    let link_for_over = link.clone();
    listen(link.as_ref(), MOUSEOVER, move |_event: MouseEvent| {
        show_info(NAME, &link_for_over, &info_for_over);
        show_info(TITLE, &link_for_over, &info_for_over);
    });

    listen(link.as_ref(), MOUSEOUT, move |_event: MouseEvent| {
        hide_info(&info);
    });
}

fn bind_info_links(info: &HtmlElement) {
    for link in node_list(INFO_LINKS_SELECTOR) {
        bind_info_link(link.dyn_into().expect("info link"), info.clone());
    }
}

fn decorate_external_links() {
    for link in node_list(NOT_PROJECT_SELECTOR) {
        let link: HtmlElement = link.dyn_into().expect("external link");
        link.set_attribute(TARGET, BLANK).expect("link target");
        link.set_attribute(REL, NOOPENER).expect("link rel");
    }
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
        bind_info_links(&info);
        decorate_external_links();
        Self::shuffle_project_links();
        Self
    }
}
