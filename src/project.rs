use stdweb::traits::*;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{document, window};
use crate::util::{get_hash, query_selector};

use crate::constants::{DATA_PROJECT, DATA_SCROLL, EMPTY, PROJECT_SELECTOR};

fn show(hash: String, scroll_top: &mut Option<f64>) {
    let body = document().body().unwrap();
    let selector = &format!(".projects .project.{}", hash);
    if document().query_selector(selector).unwrap().is_some() {
        let top = window().page_y_offset();
        body.set_attribute(DATA_PROJECT, &hash).unwrap();
        *scroll_top = Some(top);
        query_selector(PROJECT_SELECTOR).set_scroll_top(top)
    }
}

fn hide(scroll_top: &mut Option<f64>) {
    let body = document().body().unwrap();
    let top = scroll_top.take().unwrap_or(0.0);
    body.remove_attribute(DATA_PROJECT);
    if let Some(document_element) = document().document_element() {
        document_element.set_scroll_top(top);
    }
    body.set_scroll_top(top);
    body.remove_attribute(DATA_SCROLL);
}

fn toggle(scroll_top: &mut Option<f64>) {
    let hash = get_hash();
    if hash != EMPTY {
        show(hash, scroll_top)
    } else {
        hide(scroll_top)
    }
}

pub(crate) struct ToggleProject;

impl ToggleProject {
    pub(crate) fn new() -> Self {
        let mut scroll_top = None;
        toggle(&mut scroll_top);
        let toggle_project_event = move |_event: HashChangeEvent| toggle(&mut scroll_top);
        window().add_event_listener(toggle_project_event);

        Self
    }
}
