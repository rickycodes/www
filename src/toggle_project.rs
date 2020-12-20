use stdweb::traits::*;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{document, window};
use crate::util::{get_hash, query_selector};

use crate::constants::{DATA_PROJECT, DATA_SCROLL, EMPTY, PROJECT_SELECTOR};

fn show(hash: String, scrolls: &mut Vec<f64>) {
    let body = document().body().unwrap();
    let document_element = document().document_element();
    let selector = &format!(".projects .project.{}", hash);
    let query = document().query_selector(selector).unwrap();
    if query.is_some() {
        let top = Some(window().page_y_offset())
            .unwrap_or_else(|| document_element.unwrap().scroll_top());
        body.set_attribute(DATA_PROJECT, &hash).unwrap();
        scrolls.push(top);
        query_selector(PROJECT_SELECTOR).set_scroll_top(top)
    }
}

fn hide(scrolls: &mut Vec<f64>) {
    let body = document().body().unwrap();
    let document_element = document().document_element();
    let top = *scrolls.last().unwrap_or(&0.00);
    body.remove_attribute(DATA_PROJECT);
    document_element.unwrap().set_scroll_top(top);
    body.set_scroll_top(top);
    body.remove_attribute(DATA_SCROLL)
}

fn toggle(scrolls: &mut Vec<f64>) {
    let hash = get_hash();
    if hash != EMPTY {
        self::show(hash, scrolls)
    } else {
        self::hide(scrolls)
    }
}

pub(crate) struct ToggleProject();

impl ToggleProject {
    pub(crate) fn new() -> ToggleProject {
        let mut scrolls = Vec::new();
        self::toggle(&mut scrolls);
        let toggle_project_event = move |_event: HashChangeEvent| self::toggle(&mut scrolls);
        window().add_event_listener(toggle_project_event);

        ToggleProject()
    }
}
