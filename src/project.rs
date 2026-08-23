use crate::util::{get_hash, query_selector};
use stdweb::traits::*;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{document, window};

use crate::constants::{
    ACTIVE, ACTIVE_PROJECT_SELECTOR, CLASS, DATA_PROJECT, DATA_SCROLL, EMPTY, PROJECT_SELECTOR,
};

fn set_active_project(selector: Option<&str>) {
    if let Some(active_project) = document().query_selector(ACTIVE_PROJECT_SELECTOR).unwrap() {
        let class = active_project
            .get_attribute(CLASS)
            .unwrap_or_default()
            .split_whitespace()
            .filter(|class| *class != ACTIVE)
            .collect::<Vec<_>>()
            .join(" ");
        active_project.set_attribute(CLASS, &class).unwrap();
    }

    if let Some(selector) = selector {
        if let Some(project) = document().query_selector(selector).unwrap() {
            let class = project.get_attribute(CLASS).unwrap_or_default();
            project
                .set_attribute(CLASS, &format!("{} {}", class, ACTIVE))
                .unwrap();
        }
    }
}

fn show(hash: String, scroll_top: &mut Option<f64>) {
    let body = document().body().unwrap();
    let selector = &format!(".projects .project.{}", hash);
    if document().query_selector(selector).unwrap().is_some() {
        set_active_project(Some(selector));
        let top = window().page_y_offset();
        body.set_attribute(DATA_PROJECT, &hash).unwrap();
        *scroll_top = Some(top);
        query_selector(PROJECT_SELECTOR).set_scroll_top(top)
    }
}

fn hide(scroll_top: &mut Option<f64>) {
    let body = document().body().unwrap();
    set_active_project(None);
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
